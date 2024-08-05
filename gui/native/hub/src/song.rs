use revelation_mobile_midi_to_mml::MmlSong;
use rinf::debug_print;
use crate::{converter, messages::types::{SignalMmlSongOptions, SignalMmlSongStatus, SignalMmlTrack}};

pub struct SongState {
    pub song: Option<MmlSong>,
} 

impl SongState {
    pub fn new() -> Self {
        Self {
            song: None,
        }
    }

    pub fn set_song(&mut self, song: MmlSong) {
        self.song = Some(song);
    }

    pub fn get_signal_mml_song_status(&self) -> Option<SignalMmlSongStatus> {
        if let Some(song) = self.song.as_ref() {
            let song_options = converter::mml_song_options_to_signal(&song.options);
            let tracks = converter::mml_song_tracks_to_signal(&song.tracks);

            let song_status = SignalMmlSongStatus {
                song_options: Some(song_options),
                tracks,
            };

            return Some(song_status)
        } else {
            debug_print!("[get_signal_mml_song_status] song_state is None");
        }

        None
    }

    pub fn set_song_options_by_signal(&mut self, options: &SignalMmlSongOptions) -> Result<(), String> {
        if let Some(song) = self.song.as_mut() {
            let song_options = converter::signal_to_mml_song_options(options);

            if let Ok(_) = song.set_song_options(song_options) {
                return Ok(())
            }
        }

        Err(String::from("[set_song_options_by_signal] Song is None"))
    }

    pub fn get_list_track_signal(&self) -> Option<Vec<SignalMmlTrack>> {
        if let Some(song) = self.song.as_ref() {
            let list_track_signal = converter::mml_song_tracks_to_signal(&song.tracks);
            return Some(list_track_signal)
        }

        None
    }

    pub fn split_track(&mut self, index: usize) -> Result<(), String> {
        if let Some(song) = self.song.as_mut() {
            if let Ok(_) = song.split_track(index) {
                return Ok(())
            } else {
                return Err(format!("Cannot split track by index {}", index));
            }
        }

        Err(String::from("[split_track] Cannot get song state"))
    }

    pub fn merge_tracks(&mut self, index_a: usize, index_b: usize) -> Result<(), String> {
        if let Some(song) = self.song.as_mut() {
            if let Ok(_) = song.merge_tracks(index_a, index_b) {
                return Ok(())
            } else {
                return Err(format!("Cannot merge track by index_a={} index_b={}", index_a, index_b));
            }
        }

        Err(String::from("[merge_track] Cannot get song state"))
    }
}
