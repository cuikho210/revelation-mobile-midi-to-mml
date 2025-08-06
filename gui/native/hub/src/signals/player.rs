use rinf::{DartSignal, DartSignalBinary, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SignalPiece)]
pub enum SignalPlayStatus {
    Play,
    Pause,
    Stop,
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
pub struct SignalMmlNoteOn {
    pub track_index: u32,
    pub char_index: u32,
    pub char_length: u32,
}

#[derive(Serialize, RustSignal)]
pub struct SignalOnTrackEnd {
    pub track_index: u32,
}
