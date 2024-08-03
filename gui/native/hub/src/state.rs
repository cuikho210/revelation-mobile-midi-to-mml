use once_cell::sync::Lazy;
use revelation_mobile_midi_to_mml::MmlSong;
use lib_player::{MmlPlayer, MmlPlayerOptions};
use std::{path::PathBuf, sync::{Arc, Mutex}};
use crate::{converter, messages::types::{SignalMmlSongOptions, SignalMmlSongStatus, SignalMmlTrack}};

pub const SONG: Lazy<Arc<Mutex<Option<MmlSong>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

pub const PLAYER: Lazy<Arc<Mutex<MmlPlayer>>> = Lazy::new(|| Arc::new(Mutex::new(
    MmlPlayer::new(MmlPlayerOptions {
        soundfont_path: vec![
            PathBuf::from("/home/cuikho210/Documents/soundfonts/FluidR3_GM.sf2")
        ]
    })
)));

pub fn set_song(song: MmlSong) -> Result<(), String> {
    if let Ok(mut song_state) = SONG.lock() {
        *song_state = Some(song);
        return Ok(())
    }

    Err(String::from("Cannot get song state"))
}

pub fn get_signal_mml_song_status() -> Option<SignalMmlSongStatus> {
    if let Ok(song_state) = SONG.lock() {
        if let Some(song) = song_state.as_ref() {
            let song_options = converter::mml_song_options_to_signal(&song.options);
            let tracks = converter::mml_song_tracks_to_signal(&song.tracks);

            let song_status = SignalMmlSongStatus {
                song_options: Some(song_options),
                tracks,
            };

            return Some(song_status)
        }
    }

    None
}

pub fn set_song_options_by_signal(options: &SignalMmlSongOptions) -> Result<(), String> {
    if let Ok(mut song_state) = SONG.lock() {
        if let Some(song) = song_state.as_mut() {
            let song_options = converter::signal_to_mml_song_options(options);
            song.set_song_options(song_options);

            return Ok(())
        }
    }

    Err(String::from("Cannot get song state"))
}

pub fn get_list_track_signal() -> Option<Vec<SignalMmlTrack>> {
    if let Ok(song_state) = SONG.lock() {
        if let Some(song) = song_state.as_ref() {
            let list_track_signal = converter::mml_song_tracks_to_signal(&song.tracks);
            return Some(list_track_signal)
        }
    }

    None
}

pub fn split_track(index: usize) -> Result<(), String> {
    if let Ok(mut song_state) = SONG.lock() {
        if let Some(song) = song_state.as_mut() {
            song.split_track(index);
            return Ok(())
        }
    }

    Err(String::from("Cannot get song state"))
}

pub fn merge_tracks(index_a: usize, index_b: usize) -> Result<(), String> {
    if let Ok(mut song_state) = SONG.lock() {
        if let Some(song) = song_state.as_mut() {
            song.merge_tracks(index_a, index_b);
            return Ok(())
        }
    }

    Err(String::from("Cannot get song state"))
}
