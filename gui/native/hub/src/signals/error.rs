use rinf::{RustSignal, SignalPiece};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, SignalPiece)]
pub enum ErrorFrom {
    LoadSong,
    SplitTrack,
    MergeTracks,
    EqualizeTracks,
    RenameTrack,
}

#[derive(Debug, Clone, Serialize, RustSignal)]
pub struct ToastErrorSignal {
    pub title: String,
    pub content: String,
    pub source: Option<ErrorFrom>,
}
