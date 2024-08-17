use std::{
    path::{PathBuf, Path}, sync::{Arc, Mutex, RwLock},
    thread::{self, JoinHandle}, time::{Duration, Instant},
};
use cpal::{Stream, traits::StreamTrait};
use revelation_mobile_midi_to_mml::{Instrument, MmlSong};
use crate::{Parser, Synth, SynthOutputConnection, TrackPlayer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackStatus {
    PLAY,
    PAUSE,
    STOP,
}

#[derive(Debug, Clone)]
pub struct NoteOnCallbackData {
    pub track_index: usize,
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
    pub synth: Synth,
    pub stream: CpalStreamWrapper,
    pub connection: SynthOutputConnection,
    pub tracks: Vec<Arc<Mutex<TrackPlayer>>>,
    pub playback_status: Arc<RwLock<PlaybackStatus>>,

    time_start: Option<Instant>,
    time_pause: Option<Instant>,
}

impl MmlPlayer {
    pub fn new(options: MmlPlayerOptions) -> Self {
        let time = Instant::now();

        let mut synth = Synth::new();
        synth.load_soundfont_from_file_parallel(options.soundfont_path).unwrap();
        let (stream, connection) = synth.new_stream();

        log_initialize_synth(time.elapsed());
        
        Self {
            synth,
            stream: CpalStreamWrapper { stream },
            connection,
            tracks: Vec::new(),
            playback_status: Arc::new(RwLock::new(PlaybackStatus::STOP)),
            time_start: None,
            time_pause: None,
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
        let mut handles: Vec<JoinHandle<TrackPlayer>> = Vec::new();
        let mut tracks: Vec<Arc<Mutex<TrackPlayer>>> = Vec::new();

        let time = Instant::now();
        let track_length = mmls.len();
        let mut char_length: usize = 0;

        for mml in mmls {
            let conn = self.connection.clone();
            char_length += mml.0.len();

            let index = handles.len();
            let playback_status = self.playback_status.clone();

            let handle = thread::spawn::<_, TrackPlayer>(move || {
                let parser = Parser::parse(index, mml.0);

                TrackPlayer::from_parser(
                    index,
                    parser,
                    playback_status,
                    mml.1,
                    conn,
                )
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

    pub fn play(
        &mut self,
        note_on_callback: Option<Arc<fn(NoteOnCallbackData)>>,
        track_end_callback: Option<Arc<fn(usize)>>,
    ) {
        self.stream.stream.play().unwrap();

        {
            let mut guard = self.playback_status.write().unwrap();
            *guard = PlaybackStatus::PLAY;
        }

        let time_start = self.get_time_start();
        self.time_start = Some(time_start);

        let mut index = 0;
        for track in self.tracks.iter() {
            let parsed = track.clone();
            let note_on_callback = note_on_callback.clone();
            let track_end_callback = track_end_callback.clone();

            thread::Builder::new()
                .name(format!("Track player {}", index))
                .spawn(move || {
                    if let Ok(mut guard) = parsed.lock() {
                        guard.play(time_start, note_on_callback, track_end_callback);
                    } else {
                        eprintln!("[mml_player.play] Cannot lock Parsed track");
                    }
                }).unwrap();

            index += 1;
        }
    }

    pub fn pause(&mut self) {
        {
            let mut guard = self.playback_status.write().unwrap();
            *guard = PlaybackStatus::PAUSE;
        }

        self.time_pause = Some(Instant::now());
        self.stream.stream.pause().unwrap();
    }

    pub fn stop(&mut self) {
        {
            let mut guard = self.playback_status.write().unwrap();
            *guard = PlaybackStatus::STOP;
        }

        self.time_start = None;
        self.time_pause = None;
        self.stream.stream.pause().unwrap();
    }

    pub fn load_soundfont_from_bytes<B>(&mut self, bytes: B) -> Result<(), String>
        where B: AsRef<[u8]>,
    {
        self.synth.load_soundfont_from_bytes(bytes)
    }

    pub fn load_soundfont_from_bytes_parallel<B>(&mut self, list_bytes: Vec<B>) -> Result<(), String>
        where B: AsRef<[u8]> + Sync + Send + Clone + 'static,
    {
        self.synth.load_soundfont_from_bytes_parallel(list_bytes)
    }

    pub fn load_soundfont_from_file_parallel<P>(&mut self, paths: Vec<P>) -> Result<(), String>
        where P: AsRef<Path> + Sync + Send + Clone + 'static,
    {
        self.synth.load_soundfont_from_file_parallel(paths)
    }

    fn get_time_start(&self) -> Instant {
        if let Some(time_start) = self.time_start {
            if let Some(time_pause) = self.time_pause {
                let now = Instant::now();
                let diff = now - time_pause;
                let new_time = time_start + diff;

                return new_time;
            }
        }

        Instant::now()
    }
}

fn log_initialize_synth(duration: Duration) {
    println!("Initialized synth in {}ms", duration.as_millis());
}

fn log_parse_mmls(duration: Duration, track_length: usize, char_length: usize) {
    println!("Parsed {} tracks, {} chars in {}ms", track_length, char_length, duration.as_millis());
}
