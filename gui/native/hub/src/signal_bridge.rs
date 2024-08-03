use revelation_mobile_midi_to_mml::{MmlSong, MmlSongOptions};
use crate::{messages::{dart_to_rust::{SignalLoadSongFromPathPayload, SignalMergeTracksPayload, SignalSplitTrackPayload, SignalUpdateMmlSongOptionsPayload}, rust_to_dart::SignalUpdateMmlTracks}, state};

pub async fn listen_load_song_from_path() {
    let mut receiver = SignalLoadSongFromPathPayload::get_dart_signal_receiver();

    while let Some(signal) = receiver.recv().await {
        let midi_path = signal.message.path;

        if let Ok(song) = MmlSong::from_path(&midi_path, MmlSongOptions::default()) {
            state::set_song(song);
        } else {
            println!("[listen_load_song_from_path] Cannot load song from path {}", &midi_path);
        }
    }
}

pub async fn listen_update_mml_song_option() {
    let mut receiver = SignalUpdateMmlSongOptionsPayload::get_dart_signal_receiver();

    while let Some(signal) = receiver.recv().await {
        let song_options = signal.message.song_options;

        if let Some(song_options) = song_options {
            if let Ok(_) = state::set_song_options_by_signal(&song_options) {
                let tracks = state::get_list_track_signal();

                if let Some(tracks) = tracks {
                    SignalUpdateMmlTracks { tracks }.send_signal_to_dart(None);
                } else {
                    println!("[listen_update_mml_song_option] Cannot get tracks");
                }
            } else {
                println!("[listen_update_mml_song_option] Cannot set song options");
            }
        }
    }
}

pub async fn listen_split_track() {
    let mut receiver = SignalSplitTrackPayload::get_dart_signal_receiver();

    while let Some(signal) = receiver.recv().await {
        let track_index = signal.message.index as usize;

        if let Ok(_) = state::split_track(track_index) {
            let tracks = state::get_list_track_signal();

            if let Some(tracks) = tracks {
                SignalUpdateMmlTracks { tracks }.send_signal_to_dart(None);
            } else {
                println!("[listen_update_mml_song_option] Cannot get tracks");
            }
        } else {
            println!("[listen_update_mml_song_option] Cannot set song options");
        }
    }
}

pub async fn listen_merge_tracks() {
    let mut receiver = SignalMergeTracksPayload::get_dart_signal_receiver();

    while let Some(signal) = receiver.recv().await {
        let track_index_a = signal.message.index_a as usize;
        let track_index_b = signal.message.index_b as usize;

        if let Ok(_) = state::merge_tracks(track_index_a, track_index_b) {
            let tracks = state::get_list_track_signal();

            if let Some(tracks) = tracks {
                SignalUpdateMmlTracks { tracks }.send_signal_to_dart(None);
            } else {
                println!("[listen_update_mml_song_option] Cannot get tracks");
            }
        } else {
            println!("[listen_update_mml_song_option] Cannot set song options");
        }
    }
}

pub async fn listen_equalize_tracks() {

}

pub async fn listen_set_track_is_muted() {

}

pub async fn listen_set_song_play_status() {

}
