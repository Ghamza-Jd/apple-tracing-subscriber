use crate::apple_log::AppleLog;
use dashmap::DashMap;
use tracing_core::span;
use tracing_core::Event;
use tracing_core::Level;
use tracing_core::Metadata;
use tracing_core::Subscriber;

pub struct AppleTracingSubscriber {
    subsystem: String,
    categories: DashMap<String, (Option<Level>, AppleLog)>,
}

impl Subscriber for AppleTracingSubscriber {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        let max_level = self
            .categories
            .get(metadata.target())
            .and_then(|pair| pair.0)
            .unwrap_or_else(|| Level::TRACE);

        metadata.level() <= &max_level
    }

    fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
        todo!()
    }

    fn record(&self, span: &span::Id, values: &span::Record<'_>) {
        todo!()
    }

    fn record_follows_from(&self, span: &span::Id, follows: &span::Id) {
        todo!()
    }

    fn event(&self, event: &Event<'_>) {
        todo!()
    }

    fn enter(&self, span: &span::Id) {
        todo!()
    }

    fn exit(&self, span: &span::Id) {
        todo!()
    }
}

impl AppleTracingSubscriber {
    pub fn new(subsystem: &str) -> Self {
        Self {
            subsystem: subsystem.to_string(),
            categories: DashMap::new(),
        }
    }
}
