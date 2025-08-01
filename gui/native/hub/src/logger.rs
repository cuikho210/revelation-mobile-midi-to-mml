use crate::messages::rust_to_dart::SignalLogMessage;
use rinf::debug_print;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tokio::sync::Mutex;

pub enum LogType {
    ParseMidiInit,
    ParseMidiEnd,
    ParseMidiError,

    ParseMmlInit,
    ParseMmlEnd,

    SetSongOptionsInit,
    SetSongOptionsEnd,
    SetSongOptionsError,

    SplitTrackInit,
    SplitTrackEnd,
    SplitTrackError,

    MergeTrackInit,
    MergeTrackEnd,
    MergeTrackError,

    EqualizeTrackInit,
    EqualizeTrackEnd,
    EqualizeTrackError,

    SetPlaybackPlayInit,
    SetPlaybackPlayEnd,

    SetPlaybackPauseInit,
    SetPlaybackPauseEnd,

    SetPlaybackStopInit,
    SetPlaybackStopEnd,

    LoadSoundfontInit,
    LoadSoundfontEnd,
    LoadSoundfontError,

    Error(String),
}

impl LogType {
    pub fn get_log_key(&self) -> u8 {
        match self {
            Self::ParseMidiInit | Self::ParseMidiEnd | Self::ParseMidiError => 0,
            Self::ParseMmlInit | Self::ParseMmlEnd => 1,
            Self::SetSongOptionsInit | Self::SetSongOptionsEnd | Self::SetSongOptionsError => 2,
            Self::SplitTrackInit | Self::SplitTrackEnd | Self::SplitTrackError => 3,
            Self::MergeTrackInit | Self::MergeTrackEnd | Self::MergeTrackError => 4,
            Self::EqualizeTrackInit | Self::EqualizeTrackEnd | Self::EqualizeTrackError => 5,
            Self::SetPlaybackPlayInit | Self::SetPlaybackPlayEnd => 6,
            Self::SetPlaybackPauseInit | Self::SetPlaybackPauseEnd => 7,
            Self::SetPlaybackStopInit | Self::SetPlaybackStopEnd => 8,
            Self::LoadSoundfontInit | Self::LoadSoundfontEnd | Self::LoadSoundfontError => 9,
            Self::Error(_) => 10,
        }
    }

    pub fn get_effect_loading(&self) -> bool {
        match self {
            Self::ParseMidiInit => true,
            Self::ParseMidiEnd => false,
            Self::ParseMidiError => false,

            Self::ParseMmlInit => true,
            Self::ParseMmlEnd => false,

            Self::SetSongOptionsInit => true,
            Self::SetSongOptionsEnd => false,
            Self::SetSongOptionsError => false,

            Self::SplitTrackInit => true,
            Self::SplitTrackEnd => false,
            Self::SplitTrackError => false,

            Self::MergeTrackInit => true,
            Self::MergeTrackEnd => false,
            Self::MergeTrackError => false,

            Self::EqualizeTrackInit => true,
            Self::EqualizeTrackEnd => false,
            Self::EqualizeTrackError => false,

            Self::SetPlaybackPlayInit => true,
            Self::SetPlaybackPlayEnd => false,

            Self::SetPlaybackPauseInit => true,
            Self::SetPlaybackPauseEnd => false,

            Self::SetPlaybackStopInit => true,
            Self::SetPlaybackStopEnd => false,

            Self::LoadSoundfontInit => true,
            Self::LoadSoundfontEnd => false,
            Self::LoadSoundfontError => false,

            Self::Error(_) => false,
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Self::ParseMidiInit => "Parsing MIDI started".to_owned(),
            Self::ParseMidiEnd => "Parsing MIDI completed".to_owned(),
            Self::ParseMidiError => "Parsing MIDI error".to_owned(),

            Self::ParseMmlInit => "Parsing MML started".to_owned(),
            Self::ParseMmlEnd => "Parsing MML completed".to_owned(),

            Self::SetSongOptionsInit => "Setting song options started".to_owned(),
            Self::SetSongOptionsEnd => "Setting song options completed".to_owned(),
            Self::SetSongOptionsError => "Setting song options error".to_owned(),

            Self::SplitTrackInit => "Splitting track started".to_owned(),
            Self::SplitTrackEnd => "Splitting track completed".to_owned(),
            Self::SplitTrackError => "Splitting track error".to_owned(),

            Self::MergeTrackInit => "Merging tracks started".to_owned(),
            Self::MergeTrackEnd => "Merging tracks completed".to_owned(),
            Self::MergeTrackError => "Merging tracks error".to_owned(),

            Self::EqualizeTrackInit => "Equalizing track started".to_owned(),
            Self::EqualizeTrackEnd => "Equalizing track completed".to_owned(),
            Self::EqualizeTrackError => "Equalizing track error".to_owned(),

            Self::SetPlaybackPlayInit => "Playback play started".to_owned(),
            Self::SetPlaybackPlayEnd => "Playback play completed".to_owned(),

            Self::SetPlaybackPauseInit => "Playback pause started".to_owned(),
            Self::SetPlaybackPauseEnd => "Playback pause completed".to_owned(),

            Self::SetPlaybackStopInit => "Playback stop started".to_owned(),
            Self::SetPlaybackStopEnd => "Playback stop completed".to_owned(),

            Self::LoadSoundfontInit => "Loading soundfont started".to_owned(),
            Self::LoadSoundfontEnd => "Loading soundfont completed".to_owned(),
            Self::LoadSoundfontError => "Loading soundfont error".to_owned(),

            Self::Error(err) => err.to_owned(),
        }
    }
}

struct LogData {
    value: bool,
    instant: Instant,
}

pub struct Logger {
    loading_state: HashMap<u8, LogData>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            loading_state: HashMap::new(),
        }
    }

    pub fn log(&mut self, log_type: LogType) {
        let log_key = log_type.get_log_key();
        let to_effect = log_type.get_effect_loading();
        let mut message = log_type.get_message();

        if let Some(state) = self.loading_state.get_mut(&log_key) {
            state.value = to_effect;

            if to_effect {
                state.instant = Instant::now();
            } else {
                message = format!("{} in {} ms", message, state.instant.elapsed().as_millis(),);
            }
        } else {
            self.loading_state.insert(
                log_key,
                LogData {
                    value: to_effect,
                    instant: Instant::now(),
                },
            );
        }

        debug_print!("[logger::Logger.log] {}", &message);
        self.send_logging_signal(message);
    }

    fn send_logging_signal(&self, message: String) {
        let is_loading = self.get_global_loading_state();

        SignalLogMessage {
            message,
            is_loading,
        }
        .send_signal_to_dart();
    }

    fn get_global_loading_state(&self) -> bool {
        for (_, state) in self.loading_state.iter() {
            if state.value {
                return true;
            }
        }

        false
    }
}

pub async fn log(logger_state: Arc<Mutex<Logger>>, log_type: LogType) {
    let mut logger = logger_state.lock().await;
    logger.log(log_type);
}
