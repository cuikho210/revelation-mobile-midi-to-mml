//! This `hub` crate is the
//! entry point of the Rust logic.

mod messages;
mod song;
mod player;
mod converter;
mod signal_bridge;

use tokio;
use player::PlayerState;
use song::SongState;
use std::sync::Arc;
use tokio::sync::Mutex;

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    let song: Arc<Mutex<SongState>> = Arc::new(Mutex::new(SongState::new()));
    let player: Arc<Mutex<PlayerState>> = Arc::new(Mutex::new(PlayerState::new()));

    tokio::spawn(signal_bridge::listen_load_song_from_path(song.clone()));
    tokio::spawn(signal_bridge::listen_update_mml_song_option(song.clone()));
    tokio::spawn(signal_bridge::listen_split_track(song.clone()));
    tokio::spawn(signal_bridge::listen_merge_tracks(song.clone()));
    tokio::spawn(signal_bridge::listen_equalize_tracks(song.clone()));
    tokio::spawn(signal_bridge::listen_rename_tracks(song.clone()));
}
