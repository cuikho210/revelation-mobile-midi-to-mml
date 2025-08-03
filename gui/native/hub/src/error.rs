use anyhow::Error;
use rinf::RustSignal;

use crate::signals::error::{ErrorFrom, ToastErrorSignal};

pub fn send_error_signal(e: Error, title: impl ToString, source: Option<ErrorFrom>) {
    ToastErrorSignal {
        title: title.to_string(),
        content: e.to_string(),
        source,
    }
    .send_signal_to_dart();
}
