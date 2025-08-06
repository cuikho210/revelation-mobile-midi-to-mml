use rinf::{RustSignal, SignalPiece};
use serde::Serialize;
use tracing::Level;

#[derive(Serialize, SignalPiece)]
pub enum SignalLogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl From<Level> for SignalLogLevel {
    fn from(value: Level) -> Self {
        match value {
            Level::TRACE => Self::Trace,
            Level::DEBUG => Self::Debug,
            Level::INFO => Self::Info,
            Level::WARN => Self::Warn,
            Level::ERROR => Self::Error,
        }
    }
}

#[derive(Serialize, RustSignal)]
pub struct SignalLog {
    pub level: SignalLogLevel,
    pub content: String,
}
