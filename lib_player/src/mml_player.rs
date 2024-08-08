use std::{
    path::PathBuf, sync::{Arc, Mutex},
    thread::{self, JoinHandle}, time::{Duration, Instant},
};
use cpal::Stream;
use revelation_mobile_midi_to_mml::{Instrument, MmlSong};
use crate::{parser::PlaybackStatus, Parser, Synth, SynthOutputConnection};

#[derive(Debug, Clone)]
pub struct NoteOnCallbackData {
    pub char_index: usize,
    pub char_length: usize,
}

#[derive(Debug, Clone)]
pub struct MmlPlayerOptions {
    pub soundfont_path: Vec<PathBuf>,
}

pub struct CpalStreamWrapper {
    pub stream: Stream,
}
unsafe impl Send for CpalStreamWrapper {}

pub struct MmlPlayer {
    pub stream: CpalStreamWrapper,
    pub connection: SynthOutputConnection,
    pub tracks: Vec<Arc<Mutex<Parser>>>,
    pub playback_status: Arc<Mutex<PlaybackStatus>>,
}

impl MmlPlayer {
    pub fn new(options: MmlPlayerOptions) -> Self {
        let time = Instant::now();

        let synth = Synth::new();
        let (stream, connection) = synth.new_stream(options.soundfont_path);

        log_initialize_synth(time.elapsed());
        
        Self {
            stream: CpalStreamWrapper { stream },
            connection,
            tracks: Vec::new(),
            playback_status: Arc::new(Mutex::new(PlaybackStatus::STOP)),
        }
    }

    pub fn from_song(song: &MmlSong, options: MmlPlayerOptions) -> Self {
        let mmls: Vec<(String, Instrument)> = song.tracks.iter().map::<(String, Instrument), _>(|track| {
            (track.to_mml(), track.instrument.to_owned())
        }).collect();

        Self::from_mmls(mmls, options)
    }

    pub fn from_mmls(mmls: Vec<(String, Instrument)>, options: MmlPlayerOptions) -> Self {
        let mut result = Self::new(options);
        result.parse_mmls(mmls);

        result
    }

    pub fn parse_mmls(&mut self, mmls: Vec<(String, Instrument)>) {
        let mut handles: Vec<JoinHandle<Parser>> = Vec::new();
        let mut tracks: Vec<Arc<Mutex<Parser>>> = Vec::new();

        let time = Instant::now();
        let track_length = mmls.len();
        let mut char_length: usize = 0;

        for mml in mmls {
            let conn = self.connection.clone();
            char_length += mml.0.len();

            let playback_status = self.playback_status.clone();

            let handle = thread::spawn::<_, Parser>(move || {
                Parser::parse(mml.0, mml.1, conn, playback_status)
            });
            handles.push(handle);
        }

        for handle in handles {
            let parsed = handle.join().unwrap();
            tracks.push(Arc::new(Mutex::new(parsed)));
        }

        log_parse_mmls(time.elapsed(), track_length, char_length);
        self.tracks = tracks;
    }

    pub fn play(&self, note_on_callback: Option<Arc<fn(NoteOnCallbackData)>>) {
        let mut guard = self.playback_status.lock().unwrap();
        *guard = PlaybackStatus::PLAY;
        drop(guard);

        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for track in self.tracks.iter() {
            let parsed = track.clone();
            let callback = note_on_callback.clone();
            let handle = thread::spawn(move || {
                if let Ok(mut guard) = parsed.lock() {
                    guard.play(callback);
                } else {
                    eprintln!("[mml_player.play] Cannot lock Parsed track");
                }
            });
            handles.push(handle);
        }

        // for handle in handles {
        //     handle.join().unwrap();
        // }
    }

    pub fn pause(&mut self) {
        let mut guard = self.playback_status.lock().unwrap();
        *guard = PlaybackStatus::PAUSE;
        drop(guard);

        for track in self.tracks.iter() {
            let mut guard = track.lock().unwrap();
            guard.pause();
            drop(guard);
        }
    }

    pub fn stop(&mut self) {
        let mut guard = self.playback_status.lock().unwrap();
        *guard = PlaybackStatus::STOP;
        drop(guard);

        for track in self.tracks.iter() {
            let mut guard = track.lock().unwrap();
            guard.reset_state();
            drop(guard);
        }
    }
}

fn log_initialize_synth(duration: Duration) {
    println!("Initialized synth in {}ms", duration.as_millis());
}

fn log_parse_mmls(duration: Duration, track_length: usize, char_length: usize) {
    println!("Parsed {} tracks, {} chars in {}ms", track_length, char_length, duration.as_millis());
}
