//! This module contains actors.
//! To build a solid app, avoid communicating by sharing memory.
//! Focus on message passing instead.

use anyhow::Result;

use crate::actors::{player::PlayerActor, song::SongActor};

mod common;
mod player;
mod song;

// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

/// Creates and spawns the actors in the async system.
pub async fn create_actors() -> Result<()> {
    // Though simple async tasks work, using the actor model
    // is highly recommended for state management
    // to achieve modularity and scalability in your app.
    // Actors keep ownership of their state and run in their own loops,
    // handling messages from other actors or external sources,
    // such as websockets or timers.

    let player = PlayerActor::spawn()?;
    SongActor::spawn(player);

    Ok(())
}
