use std::collections::HashMap;

use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece)]
pub struct SignalMmlSongOptions {
    pub auto_boot_velocity: bool,
    pub auto_equalize_note_length: bool,
    pub velocity_min: u32,
    pub velocity_max: u32,
    pub min_gap_for_chord: u32,
    pub smallest_unit: u32,
}

#[derive(Serialize, SignalPiece)]
pub struct SignalInstrument {
    pub name: String,
    pub instrument_id: u32,
    pub midi_channel: u32,
}

#[derive(Serialize, SignalPiece)]
pub struct SignalMmlTrack {
    pub index: u32,
    pub name: String,
    pub instrument: SignalInstrument,
    pub mml: String,
    pub mml_note_length: u32,
}

#[derive(Serialize, SignalPiece)]
pub struct SignalMmlSongStatus {
    pub song_options: SignalMmlSongOptions,
    pub tracks: Vec<SignalMmlTrack>,
}

#[derive(Deserialize, DartSignal)]
pub struct SignalLoadSongFromPathRequest {
    pub path: String,
}

#[derive(Serialize, RustSignal)]
pub struct SignalLoadSongFromPathResponse {
    pub song_status: Option<SignalMmlSongStatus>,
}

#[derive(Deserialize, DartSignal)]
pub struct SignalUpdateMmlSongOptionsRequest {
    pub song_options: SignalMmlSongOptions,
}

#[derive(Deserialize, DartSignal)]
pub struct SignalSplitTrackRequest {
    pub index: u32,
}

#[derive(Deserialize, DartSignal)]
pub struct SignalMergeTracksRequest {
    pub index_a: u32,
    pub index_b: u32,
}

#[derive(Deserialize, DartSignal)]
pub struct SignalRenameTrackRequest {
    pub index: u32,
    pub name: String,
}

#[derive(Deserialize, DartSignal)]
pub struct SignalEqualizeTracksRequest {
    pub index_a: u32,
    pub index_b: u32,
}

#[derive(Serialize, RustSignal)]
pub struct SignalUpdateMmlTracks {
    pub tracks: Vec<SignalMmlTrack>,
}

#[derive(Deserialize, DartSignal)]
pub struct SignalApplyKeymap {
    pub track_index: u32,
    pub keymap: HashMap<u8, u8>,
}
