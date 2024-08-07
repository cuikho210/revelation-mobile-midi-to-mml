use std::sync::Arc;
use tokio::sync::Mutex;
use revelation_mobile_midi_to_mml::{MmlSong, MmlSongOptions};
use crate::{
    messages::{
        dart_to_rust::{SignalEqualizeTracksPayload, SignalLoadSongFromPathPayload, SignalMergeTracksPayload, SignalRenameTrackPayload, SignalSplitTrackPayload, SignalUpdateMmlSongOptionsPayload},
        rust_to_dart::{SignalLoadSongFromPathResponse, SignalUpdateMmlTracks},
    },
    song::SongState,
};
use rinf::{debug_print, RinfError};

pub async fn listen_load_song_from_path(song_state: Arc<Mutex<SongState>>) -> Result<(), RinfError> {
    let mut receiver = SignalLoadSongFromPathPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let midi_path = signal.message.path;

        if let Ok(song) = MmlSong::from_path(&midi_path, MmlSongOptions::default()) {
            let mut guard = song_state.lock().await;
            guard.set_song(song);
            let song_status = guard.get_signal_mml_song_status();

            debug_print!("[listen_load_song_from_path] Loaded song from {}", &midi_path);
            SignalLoadSongFromPathResponse { song_status }.send_signal_to_dart();
        } else {
            debug_print!("[listen_load_song_from_path] Cannot load song from path {}", &midi_path);
        }
    }

    Ok(())
}

pub async fn listen_update_mml_song_option(song_state: Arc<Mutex<SongState>>) -> Result<(), RinfError> {
    let mut receiver = SignalUpdateMmlSongOptionsPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let song_options = signal.message.song_options;

        if let Some(song_options) = song_options {
            let mut song = song_state.lock().await;

            match song.set_song_options_by_signal(&song_options) {
                Ok(_) => {
                    let tracks = song.get_list_track_signal();

                    if let Some(tracks) = tracks {
                        SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
                    } else {
                        debug_print!("[listen_update_mml_song_option] Cannot get tracks");
                    }
                }
                Err(err_message) => {
                    debug_print!("[listen_update_mml_song_option] {}", err_message);
                }
            }
        }
    }

    Ok(())
}

pub async fn listen_split_track(song_state: Arc<Mutex<SongState>>) -> Result<(), RinfError> {
    let mut receiver = SignalSplitTrackPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let track_index = signal.message.index as usize;
        let mut song = song_state.lock().await;

        match song.split_track(track_index) {
            Ok(_) => {
                let tracks = song.get_list_track_signal();

                if let Some(tracks) = tracks {
                    SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
                } else {
                    debug_print!("[listen_update_mml_song_option] Cannot get tracks");
                }
            }
            Err(err) => {
                debug_print!("[listen_update_mml_song_option] {}", err);
            }
        }
    }

    Ok(())
}

pub async fn listen_merge_tracks(song_state: Arc<Mutex<SongState>>) -> Result<(), RinfError> {
    let mut receiver = SignalMergeTracksPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let track_index_a = signal.message.index_a as usize;
        let track_index_b = signal.message.index_b as usize;
        let mut song = song_state.lock().await;

        match song.merge_tracks(track_index_a, track_index_b) {
            Ok(_) => {
                let tracks = song.get_list_track_signal();

                if let Some(tracks) = tracks {
                    SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
                } else {
                    debug_print!("[listen_update_mml_song_option] Cannot get tracks");
                }
            }
            Err(err) => {
                debug_print!("[listen_update_mml_song_option] {}", err);
            }
        }
    }

    Ok(())
}

pub async fn listen_equalize_tracks(song_state: Arc<Mutex<SongState>>) -> Result<(), RinfError> {
    let mut receiver = SignalEqualizeTracksPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let track_index_a = signal.message.index_a as usize;
        let track_index_b = signal.message.index_b as usize;
        let mut song = song_state.lock().await;

        match song.equalize_tracks(track_index_a, track_index_b) {
            Ok(_) => {
                let tracks = song.get_list_track_signal();

                if let Some(tracks) = tracks {
                    SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
                } else {
                    debug_print!("[listen_update_mml_song_option] Cannot get tracks");
                }
            }
            Err(err) => {
                debug_print!("[listen_update_mml_song_option] {}", err);
            }
        }
    }

    Ok(())
}

pub async fn listen_rename_tracks(song_state: Arc<Mutex<SongState>>) -> Result<(), RinfError> {
    let mut receiver = SignalRenameTrackPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let track_index = signal.message.index as usize;
        let new_track_name = signal.message.name;
        let mut song = song_state.lock().await;

        match song.rename_track(track_index, new_track_name) {
            Ok(_) => {
                let tracks = song.get_list_track_signal();

                if let Some(tracks) = tracks {
                    SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
                } else {
                    debug_print!("[listen_update_mml_song_option] Cannot get tracks");
                }
            }
            Err(err) => {
                debug_print!("[listen_update_mml_song_option] {}", err);
            }
        }
    }

    Ok(())
}

pub async fn listen_set_track_is_muted() -> Result<(), RinfError> {

    Ok(())
}

pub async fn listen_set_song_play_status() -> Result<(), RinfError> {

    Ok(())
}
