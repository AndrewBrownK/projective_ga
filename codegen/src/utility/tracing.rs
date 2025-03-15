use tracing_subscriber::fmt::{FormatEvent, FmtContext};
use tracing::{Event, Subscriber};
use std::fmt;
use tracing_subscriber::fmt::format::{Format, Writer};

pub struct DebuggableCopyPasta<F> { pub default_formatter: F }

impl DebuggableCopyPasta<Format> {
    pub fn new() -> Self {
        // TODO pretty or default? or compact? which is preferable?
        DebuggableCopyPasta { default_formatter: Format::default() }
    }
}

impl<F, S, N> FormatEvent<S, N> for DebuggableCopyPasta<F>
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: for<'a> tracing_subscriber::fmt::FormatFields<'a> + 'static,
    F: FormatEvent<S, N>,
{
    fn format_event(&self, ctx: &FmtContext<'_, S, N>, mut writer: Writer<'_>, event: &Event<'_>) -> fmt::Result {
        if let Some(value) = event.fields().find(|f| f.name() == "debuggable_copy_pasta") {
            write!(writer, "{}", value)?;
        } else {
            self.default_formatter.format_event(ctx, writer, event)?;
        }
        Ok(())
    }
}