use flutter_rust_bridge::frb;
pub use midi_to_mml::*;

#[frb(mirror(SongOptions))]
pub struct _SongOptions {
    pub is_split_track: bool,
    pub merge_track: Vec<(usize, usize)>,
}

#[frb(mirror(Song))]
pub struct _Song {
    pub ppq: u16,
    pub bpm: u16,
    pub tracks: Vec<Track>,
}
