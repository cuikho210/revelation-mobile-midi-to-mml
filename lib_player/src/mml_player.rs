use std::{
    path::PathBuf, sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle}, time::{Duration, Instant},
};
use cpal::Stream;
use revelation_mobile_midi_to_mml::{Instrument, MmlSong};
use crate::{Parser, Synth, SynthOutputConnection};

pub struct NoteOnCallbackData {
    pub char_index: usize,
    pub char_length: usize,
}

pub struct MmlPlayerOptions {
    pub soundfont_path: Vec<PathBuf>,
}

pub struct MmlPlayer {
    pub synth: Synth,
    pub stream: Stream,
    pub connection: SynthOutputConnection,
    pub tracks: Vec<Arc<Mutex<Parser>>>,
}

impl MmlPlayer {
    pub fn new(options: MmlPlayerOptions) -> Self {
        let time = Instant::now();

        let synth = Synth::new();
        let (stream, connection) = synth.new_stream(options.soundfont_path);

        log_initialize_synth(time.elapsed());
        
        Self {
            synth,
            stream,
            connection,
            tracks: Vec::new(),
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

            let handle = thread::spawn::<_, Parser>(move || {
                Parser::parse(mml.0, mml.1, conn)
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

    pub fn play(&self, note_on_callback: Option<fn(NoteOnCallbackData)>) {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        let (note_on_tx, note_on_rx) = mpsc::channel::<NoteOnCallbackData>();

        for track in self.tracks.iter() {
            let parsed = track.clone();
            let tx = note_on_tx.clone();
            let handle = thread::spawn(move || {
                if let Ok(mut guard) = parsed.lock() {
                    guard.play(tx);
                } else {
                    eprintln!("[mml_player.play] Cannot lock Parsed track");
                }
            });
            handles.push(handle);
        }

        for received in note_on_rx {
            if let Some(callback) = note_on_callback {
                callback(received);
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn log_initialize_synth(duration: Duration) {
    println!("Initialized synth in {}ms", duration.as_millis());
}

fn log_parse_mmls(duration: Duration, track_length: usize, char_length: usize) {
    println!("Parsed {} tracks, {} chars in {}ms", track_length, char_length, duration.as_millis());
}
