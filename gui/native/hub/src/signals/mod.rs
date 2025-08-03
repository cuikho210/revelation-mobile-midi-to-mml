use rinf::{DartSignal, DartSignalBinary, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

pub mod error;

#[derive(Serialize, Deserialize, SignalPiece)]
pub enum SignalPlayStatus {
    Play,
    Pause,
    Stop,
}

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

// TODO: Implement mute a track
// #[derive(Deserialize, DartSignal)]
// pub struct SignalSetTrackIsMutedRequest {
//     pub index: u32,
//     pub is_muted: bool,
// }

#[derive(Deserialize, DartSignal)]
pub struct SignalSetSongPlayStatusRequest {
    pub status: SignalPlayStatus,
}

#[derive(Deserialize, DartSignalBinary)]
pub struct SignalLoadSoundfontRequest;

#[derive(Serialize, RustSignal)]
pub struct SignalLoadSongFromPathResponse {
    pub song_status: Option<SignalMmlSongStatus>,
}

#[derive(Serialize, RustSignal)]
pub struct SignalUpdateMmlTracks {
    pub tracks: Vec<SignalMmlTrack>,
}

#[derive(Serialize, RustSignal)]
pub struct SignalMmlNoteOn {
    pub track_index: u32,
    pub char_index: u32,
    pub char_length: u32,
}

#[derive(Serialize, RustSignal)]
pub struct SignalOnTrackEnd {
    pub track_index: u32,
}

// TODO: Implement logging
// #[derive(Serialize, RustSignal)]
// pub struct SignalLogMessage {
//     pub message: String,
//     pub is_loading: bool,
// }
