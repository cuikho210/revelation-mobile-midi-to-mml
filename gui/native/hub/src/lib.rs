mod messages;
mod state;
mod signal_bridge;
mod converter;
mod player;

use tokio_with_wasm::tokio;

async fn main() {
    tokio::spawn(signal_bridge::listen_load_song_from_path());
    tokio::spawn(signal_bridge::listen_update_mml_song_option());
    tokio::spawn(signal_bridge::listen_split_track());
    tokio::spawn(signal_bridge::listen_merge_tracks());
    tokio::spawn(signal_bridge::listen_equalize_tracks());
    tokio::spawn(signal_bridge::listen_set_track_is_muted());
    tokio::spawn(signal_bridge::listen_set_song_play_status());
}
