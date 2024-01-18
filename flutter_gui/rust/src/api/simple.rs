use midi_to_mml::{Song, SongOptions};

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn parse_midi_to_mml(bytes: Vec<u8>) -> Song {
    Song::from_bytes(bytes, SongOptions::default()).unwrap()
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
