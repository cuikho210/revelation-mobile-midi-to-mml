//! Reference: https://burgers.io/custom-logging-in-rust-using-tracing

use rinf::RustSignal;
use tracing::Level;
use tracing_subscriber::Layer;

use crate::signals::logging::SignalLog;

struct ReportToDartVisitor(Level);

impl From<Level> for ReportToDartVisitor {
    fn from(value: Level) -> Self {
        Self(value)
    }
}

impl tracing::field::Visit for ReportToDartVisitor {
    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        SignalLog {
            level: self.0.into(),
            content: format!("[error] {}: {}", field.name(), value),
        }
        .send_signal_to_dart();
    }

    fn record_debug(&mut self, _field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        SignalLog {
            level: self.0.into(),
            content: format!("{:?}", value),
        }
        .send_signal_to_dart();
    }
}

pub struct ReportToDartLayer;

impl<S> Layer<S> for ReportToDartLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let level = event.metadata().level();
        let mut visitor: ReportToDartVisitor = level.to_owned().into();
        event.record(&mut visitor);
    }
}
