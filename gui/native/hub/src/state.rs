use midi_to_mml_cui::commands;
use revelation_mobile_midi_to_mml::{Song, SongOptions};
use tokio::sync::Mutex;
use crate::{
    messages::types::{Track, SongOptions as SignalSongOptions},
    utils,
};

pub struct State {
    song: Option<Song>,
}

pub static STATE: Mutex<State> = Mutex::const_new(State {
    song: None,
});

pub async fn get_mml() -> Vec<String> {
    let state = STATE.lock().await;
    let song = state.song.as_ref().unwrap();

    song.tracks.iter().map(|track| track.to_mml()).collect()
}

pub async fn merge_tracks(index_a: usize, index_b: usize) -> Vec<Track> {
    let mut state = STATE.lock().await;
    let song = state.song.as_mut().unwrap();

    commands::merge_tracks(song, &index_a, &index_b);
    utils::get_tracks_from_song(song)
}

pub async fn split_track(index: usize) -> Vec<Track> {
    let mut state = STATE.lock().await;
    let song = state.song.as_mut().unwrap();

    commands::split_track(song, &index);
    utils::get_tracks_from_song(song)
}

pub async fn set_song_options(options: SignalSongOptions) {
    let mut state = STATE.lock().await;
    let song = state.song.as_mut().unwrap();

    song.set_song_options(SongOptions {
        auto_boot_velocity: options.auto_boot_velocity,
        velocity_min: options.velocity_min.try_into().unwrap(),
        velocity_max: options.velocity_max.try_into().unwrap(),
    });
}

pub async fn set_temp_song(song: Song) {
    let mut state = STATE.lock().await;
    state.song = Some(song);
}
