//! This `hub` crate is the
//! entry point of the Rust logic.

mod actors;
mod error;
mod logging;
mod signal_converter;
mod signals;

use actors::create_actors;
use rinf::{dart_shutdown, write_interface};
use tokio::spawn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::logging::ReportToDartLayer;

// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

write_interface!();

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_line_number(true)
        .finish()
        .with(ReportToDartLayer)
        .init();

    // Spawn concurrent tasks.
    // Always use non-blocking async functions like `tokio::fs::File::open`.
    // If you must use blocking code, use `tokio::task::spawn_blocking`
    // or the equivalent provided by your async library.
    spawn(create_actors());

    // Keep the main function running until Dart shutdown.
    dart_shutdown().await;
}
