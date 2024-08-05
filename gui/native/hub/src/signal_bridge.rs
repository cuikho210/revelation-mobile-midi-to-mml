use std::sync::Arc;
use tokio::sync::Mutex;
use revelation_mobile_midi_to_mml::{MmlSong, MmlSongOptions};
use crate::{
    messages::{
        dart_to_rust::{SignalLoadSongFromPathPayload, SignalMergeTracksPayload, SignalSplitTrackPayload, SignalUpdateMmlSongOptionsPayload},
        rust_to_dart::{SignalLoadSongFromPathResponse, SignalUpdateMmlTracks},
    },
    song,
};
use rinf::{debug_print, RinfError};

pub async fn listen_load_song_from_path(state: Arc<Mutex<Option<MmlSong>>>) -> Result<(), RinfError> {
    let mut receiver = SignalLoadSongFromPathPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let midi_path = signal.message.path;

        if let Ok(song) = MmlSong::from_path(&midi_path, MmlSongOptions::default()) {
            song::set_song(state.clone(), song).await;
            let song_status = song::get_signal_mml_song_status(state.clone()).await;
            debug_print!("[listen_load_song_from_path] Loaded song from {}", &midi_path);

            SignalLoadSongFromPathResponse { song_status }.send_signal_to_dart();
        } else {
            debug_print!("[listen_load_song_from_path] Cannot load song from path {}", &midi_path);
        }
    }

    Ok(())
}
//
// pub async fn listen_update_mml_song_option() {
//     if let Ok(mut receiver) = SignalUpdateMmlSongOptionsPayload::get_dart_signal_receiver() {
//         while let Some(signal) = receiver.recv().await {
//             let song_options = signal.message.song_options;
//
//             if let Some(song_options) = song_options {
//                 if let Ok(_) = song::set_song_options_by_signal(&song_options) {
//                     let tracks = song::get_list_track_signal();
//
//                     if let Some(tracks) = tracks {
//                         SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
//                     } else {
//                         debug_print!("[listen_update_mml_song_option] Cannot get tracks");
//                     }
//                 } else {
//                     debug_print!("[listen_update_mml_song_option] Cannot set song options");
//                 }
//             }
//         }
//     }
// }
//
// pub async fn listen_split_track() {
//     if let Ok(mut receiver) = SignalSplitTrackPayload::get_dart_signal_receiver() {
//         while let Some(signal) = receiver.recv().await {
//             let track_index = signal.message.index as usize;
//
//             if let Ok(_) = song::split_track(track_index) {
//                 let tracks = song::get_list_track_signal();
//
//                 if let Some(tracks) = tracks {
//                     SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
//                 } else {
//                     debug_print!("[listen_update_mml_song_option] Cannot get tracks");
//                 }
//             } else {
//                 debug_print!("[listen_update_mml_song_option] Cannot set song options");
//             }
//         }
//     }
// }
//
// pub async fn listen_merge_tracks() {
//     if let Ok(mut receiver) = SignalMergeTracksPayload::get_dart_signal_receiver() {
//         while let Some(signal) = receiver.recv().await {
//             let track_index_a = signal.message.index_a as usize;
//             let track_index_b = signal.message.index_b as usize;
//
//             if let Ok(_) = song::merge_tracks(track_index_a, track_index_b) {
//                 let tracks = song::get_list_track_signal();
//
//                 if let Some(tracks) = tracks {
//                     SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
//                 } else {
//                     debug_print!("[listen_update_mml_song_option] Cannot get tracks");
//                 }
//             } else {
//                 debug_print!("[listen_update_mml_song_option] Cannot set song options");
//             }
//         }
//     }
// }
//
// pub async fn listen_equalize_tracks() {
//
// }
//
// pub async fn listen_set_track_is_muted() {
//
// }
//
// pub async fn listen_set_song_play_status() {
//
// }
