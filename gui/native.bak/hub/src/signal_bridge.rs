use crate::{
    logger::{log, LogType, Logger},
    messages::{
        dart_to_rust::{
            SignalEqualizeTracksPayload, SignalLoadListSoundfontPayload,
            SignalLoadSongFromPathPayload, SignalLoadSoundfontPayload, SignalMergeTracksPayload,
            SignalRenameTrackPayload, SignalSetSongPlayStatusPayload, SignalSplitTrackPayload,
            SignalUpdateMmlSongOptionsPayload,
        },
        rust_to_dart::{SignalLoadSongFromPathResponse, SignalUpdateMmlTracks},
    },
    player::{parse_mmls_parallel, PlayerState},
    song::SongState,
};
use revelation_mobile_midi_to_mml::{MmlSong, MmlSongOptions};
use rinf::{debug_print, RinfError};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn listen_load_song_from_path(
    song_state: Arc<Mutex<SongState>>,
    player_state: Arc<Mutex<PlayerState>>,
    logger_state: Arc<Mutex<Logger>>,
) -> Result<(), RinfError> {
    let mut receiver = SignalLoadSongFromPathPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let midi_path = signal.message.path;
        log(logger_state.clone(), LogType::ParseMidiInit).await;

        if let Ok(song) = MmlSong::from_path(&midi_path, MmlSongOptions::default()) {
            let mut guard = song_state.lock().await;
            guard.set_song(song);
            guard.update_list_track_mml().unwrap();
            let song_status = guard.get_signal_mml_song_status();

            log(logger_state.clone(), LogType::ParseMidiEnd).await;

            let player_state = player_state.clone();
            parse_mmls_parallel(player_state, logger_state.clone(), guard.mmls.to_owned());

            SignalLoadSongFromPathResponse { song_status }.send_signal_to_dart();
        } else {
            log(logger_state.clone(), LogType::ParseMidiError).await;
            debug_print!(
                "[listen_load_song_from_path] Cannot load song from path {}",
                &midi_path
            );
        }
    }

    Ok(())
}

pub async fn listen_update_mml_song_option(
    song_state: Arc<Mutex<SongState>>,
    player_state: Arc<Mutex<PlayerState>>,
    logger_state: Arc<Mutex<Logger>>,
) -> Result<(), RinfError> {
    let mut receiver = SignalUpdateMmlSongOptionsPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let song_options = signal.message.song_options;

        if let Some(song_options) = song_options {
            log(logger_state.clone(), LogType::SetSongOptionsInit).await;

            let mut song = song_state.lock().await;

            match song.set_song_options_by_signal(&song_options) {
                Ok(_) => {
                    song.update_list_track_mml().unwrap();

                    log(logger_state.clone(), LogType::SetSongOptionsEnd).await;

                    let player_state = player_state.clone();
                    parse_mmls_parallel(player_state, logger_state.clone(), song.mmls.to_owned());

                    let tracks = song.get_list_track_signal();

                    if let Some(tracks) = tracks {
                        SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
                    } else {
                        debug_print!("[listen_update_mml_song_option] Cannot get tracks");
                    }
                }
                Err(err_message) => {
                    log(logger_state.clone(), LogType::SetSongOptionsError).await;
                    debug_print!("[listen_update_mml_song_option] {}", err_message);
                }
            }
        }
    }

    Ok(())
}

pub async fn listen_split_track(
    song_state: Arc<Mutex<SongState>>,
    player_state: Arc<Mutex<PlayerState>>,
    logger_state: Arc<Mutex<Logger>>,
) -> Result<(), RinfError> {
    let mut receiver = SignalSplitTrackPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        log(logger_state.clone(), LogType::SplitTrackInit).await;

        let track_index = signal.message.index as usize;
        let mut song = song_state.lock().await;

        match song.split_track(track_index) {
            Ok(_) => {
                song.update_list_track_mml().unwrap();

                log(logger_state.clone(), LogType::SplitTrackEnd).await;

                let player_state = player_state.clone();
                parse_mmls_parallel(player_state, logger_state.clone(), song.mmls.to_owned());

                let tracks = song.get_list_track_signal();

                if let Some(tracks) = tracks {
                    SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
                } else {
                    debug_print!("[listen_update_mml_song_option] Cannot get tracks");
                }
            }
            Err(err) => {
                log(logger_state.clone(), LogType::SplitTrackError).await;
                debug_print!("[listen_update_mml_song_option] {}", err);
            }
        }
    }

    Ok(())
}

pub async fn listen_merge_tracks(
    song_state: Arc<Mutex<SongState>>,
    player_state: Arc<Mutex<PlayerState>>,
    logger_state: Arc<Mutex<Logger>>,
) -> Result<(), RinfError> {
    let mut receiver = SignalMergeTracksPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        log(logger_state.clone(), LogType::MergeTrackInit).await;

        let track_index_a = signal.message.index_a as usize;
        let track_index_b = signal.message.index_b as usize;
        let mut song = song_state.lock().await;

        match song.merge_tracks(track_index_a, track_index_b) {
            Ok(_) => {
                song.update_list_track_mml().unwrap();

                log(logger_state.clone(), LogType::MergeTrackEnd).await;

                let player_state = player_state.clone();
                parse_mmls_parallel(player_state, logger_state.clone(), song.mmls.to_owned());

                let tracks = song.get_list_track_signal();

                if let Some(tracks) = tracks {
                    SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
                } else {
                    debug_print!("[listen_update_mml_song_option] Cannot get tracks");
                }
            }
            Err(err) => {
                log(logger_state.clone(), LogType::MergeTrackError).await;
                debug_print!("[listen_update_mml_song_option] {}", err);
            }
        }
    }

    Ok(())
}

pub async fn listen_equalize_tracks(
    song_state: Arc<Mutex<SongState>>,
    player_state: Arc<Mutex<PlayerState>>,
    logger_state: Arc<Mutex<Logger>>,
) -> Result<(), RinfError> {
    let mut receiver = SignalEqualizeTracksPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        log(logger_state.clone(), LogType::EqualizeTrackInit).await;

        let track_index_a = signal.message.index_a as usize;
        let track_index_b = signal.message.index_b as usize;
        let mut song = song_state.lock().await;

        match song.equalize_tracks(track_index_a, track_index_b) {
            Ok(_) => {
                song.update_list_track_mml().unwrap();

                log(logger_state.clone(), LogType::EqualizeTrackEnd).await;

                let player_state = player_state.clone();
                parse_mmls_parallel(player_state, logger_state.clone(), song.mmls.to_owned());

                let tracks = song.get_list_track_signal();

                if let Some(tracks) = tracks {
                    SignalUpdateMmlTracks { tracks }.send_signal_to_dart();
                } else {
                    debug_print!("[listen_update_mml_song_option] Cannot get tracks");
                }
            }
            Err(err) => {
                log(logger_state.clone(), LogType::EqualizeTrackError).await;
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

pub async fn listen_set_song_play_status(
    player_state: Arc<Mutex<PlayerState>>,
    logger_state: Arc<Mutex<Logger>>,
) -> Result<(), RinfError> {
    let mut receiver = SignalSetSongPlayStatusPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let status = signal.message.status;
        let mut player = player_state.lock().await;

        if status == 0 {
            log(logger_state.clone(), LogType::SetPlaybackPlayInit).await;

            if let Err(err) = player.play() {
                debug_print!(
                    "[listen_set_song_play_status] Failed to play player: {}",
                    err
                );
                log(logger_state.clone(), LogType::Error(err.to_string())).await;
            };

            log(logger_state.clone(), LogType::SetPlaybackPlayEnd).await;
        } else if status == 1 {
            log(logger_state.clone(), LogType::SetPlaybackPauseInit).await;

            if let Err(err) = player.pause() {
                debug_print!(
                    "[listen_set_song_play_status] Failed to pause player: {}",
                    err
                );
                log(logger_state.clone(), LogType::Error(err.to_string())).await;
            };

            log(logger_state.clone(), LogType::SetPlaybackPauseEnd).await;
        } else {
            log(logger_state.clone(), LogType::SetPlaybackStopInit).await;

            if let Err(err) = player.stop() {
                debug_print!(
                    "[listen_set_song_play_status] Failed to stop player: {}",
                    err
                );
                log(logger_state.clone(), LogType::Error(err.to_string())).await;
            }

            log(logger_state.clone(), LogType::SetPlaybackStopEnd).await;
        }
    }

    Ok(())
}

pub async fn listen_load_soundfont(
    player_state: Arc<Mutex<PlayerState>>,
    logger_state: Arc<Mutex<Logger>>,
) -> Result<(), RinfError> {
    let mut receiver = SignalLoadSoundfontPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let bytes = signal.binary;
        let mut player = player_state.lock().await;

        log(logger_state.clone(), LogType::LoadSoundfontInit).await;

        if let Err(message) = player.load_soundfont_from_bytes(bytes) {
            log(logger_state.clone(), LogType::LoadSoundfontError).await;
            debug_print!("[listen_load_list_soundfont] error: {}", message);
        } else {
            log(logger_state.clone(), LogType::LoadSoundfontEnd).await;
        }
    }

    Ok(())
}

pub async fn listen_load_list_soundfont(
    player_state: Arc<Mutex<PlayerState>>,
    logger_state: Arc<Mutex<Logger>>,
) -> Result<(), RinfError> {
    let mut receiver = SignalLoadListSoundfontPayload::get_dart_signal_receiver()?;

    while let Some(signal) = receiver.recv().await {
        let list_bytes = signal.message.list_soundfont_bytes;
        let mut player = player_state.lock().await;

        log(logger_state.clone(), LogType::LoadSoundfontInit).await;

        if let Err(message) = player.load_soundfont_from_bytes_parallel(list_bytes) {
            log(logger_state.clone(), LogType::LoadSoundfontError).await;
            debug_print!("[listen_load_list_soundfont] error: {}", message);
        } else {
            log(logger_state.clone(), LogType::LoadSoundfontEnd).await;
        }
    }

    Ok(())
}
