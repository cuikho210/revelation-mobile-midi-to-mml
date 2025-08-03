//! This `hub` crate is the
//! entry point of the Rust logic.

mod converter;
mod logger;
mod messages;
mod player;
mod signal_bridge;
mod song;

use anyhow::Result;
use logger::Logger;
use player::PlayerState;
use song::SongState;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() -> Result<()> {
    let song: Arc<Mutex<SongState>> = Arc::new(Mutex::new(SongState::new()));
    let player: Arc<Mutex<PlayerState>> = Arc::new(Mutex::new(PlayerState::new()?));
    let logger: Arc<Mutex<Logger>> = Arc::new(Mutex::new(Logger::new()));

    tokio::spawn(signal_bridge::listen_load_song_from_path(
        song.clone(),
        player.clone(),
        logger.clone(),
    ));

    tokio::spawn(signal_bridge::listen_update_mml_song_option(
        song.clone(),
        player.clone(),
        logger.clone(),
    ));

    tokio::spawn(signal_bridge::listen_split_track(
        song.clone(),
        player.clone(),
        logger.clone(),
    ));

    tokio::spawn(signal_bridge::listen_merge_tracks(
        song.clone(),
        player.clone(),
        logger.clone(),
    ));

    tokio::spawn(signal_bridge::listen_equalize_tracks(
        song.clone(),
        player.clone(),
        logger.clone(),
    ));

    tokio::spawn(signal_bridge::listen_rename_tracks(song.clone()));

    tokio::spawn(signal_bridge::listen_set_song_play_status(
        player.clone(),
        logger.clone(),
    ));

    tokio::spawn(signal_bridge::listen_load_soundfont(
        player.clone(),
        logger.clone(),
    ));

    tokio::spawn(signal_bridge::listen_load_list_soundfont(
        player.clone(),
        logger.clone(),
    ));

    Ok(())
}
