use tracing_core::span;
use tracing_core::Event;
use tracing_core::Metadata;
use tracing_core::Subscriber;

pub struct AppleTracingSubscriber;

impl Subscriber for AppleTracingSubscriber {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        todo!()
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
