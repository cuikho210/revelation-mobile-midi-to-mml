mod state;
mod messages;
mod commands;
mod utils;

use tokio_with_wasm::tokio;

rinf::write_interface!();

async fn main() {
    tokio::spawn(commands::import_midi_data());
}
