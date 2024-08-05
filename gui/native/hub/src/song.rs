use std::sync::Arc;

use revelation_mobile_midi_to_mml::MmlSong;
use rinf::debug_print;
use tokio::sync::Mutex;
use crate::{converter, messages::types::{SignalMmlSongOptions, SignalMmlSongStatus, SignalMmlTrack}};

pub async fn set_song(state: Arc<Mutex<Option<MmlSong>>>, song: MmlSong) {
    let mut guard = state.lock().await;
    *guard = Some(song);
}

pub async fn get_signal_mml_song_status(state: Arc<Mutex<Option<MmlSong>>>) -> Option<SignalMmlSongStatus> {
    let guard = state.lock().await;

    if let Some(song) = guard.as_ref() {
        let song_options = converter::mml_song_options_to_signal(&song.options);
        let tracks = converter::mml_song_tracks_to_signal(&song.tracks);
        debug_print!("{} tracks", tracks.len());

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
//
// pub fn set_song_options_by_signal(options: &SignalMmlSongOptions) -> Result<(), String> {
//     if let Ok(mut song_state) = SONG.lock() {
//         if let Some(song) = song_state.as_mut() {
//             let song_options = converter::signal_to_mml_song_options(options);
//
//             if let Ok(_) = song.set_song_options(song_options) {
//                 return Ok(())
//             }
//         }
//     }
//
//     Err(String::from("Cannot get song state"))
// }
//
// pub fn get_list_track_signal() -> Option<Vec<SignalMmlTrack>> {
//     if let Ok(song_state) = SONG.lock() {
//         if let Some(song) = song_state.as_ref() {
//             let list_track_signal = converter::mml_song_tracks_to_signal(&song.tracks);
//             return Some(list_track_signal)
//         }
//     }
//
//     None
// }
//
// pub fn split_track(index: usize) -> Result<(), String> {
//     if let Ok(mut song_state) = SONG.lock() {
//         if let Some(song) = song_state.as_mut() {
//             if let Ok(_) = song.split_track(index) {
//                 return Ok(())
//             }
//         }
//     }
//
//     Err(String::from("Cannot get song state"))
// }
//
// pub fn merge_tracks(index_a: usize, index_b: usize) -> Result<(), String> {
//     if let Ok(mut song_state) = SONG.lock() {
//         if let Some(song) = song_state.as_mut() {
//             if let Ok(_) = song.merge_tracks(index_a, index_b) {
//                 return Ok(())
//             }
//         }
//     }
//
//     Err(String::from("Cannot get song state"))
// }
