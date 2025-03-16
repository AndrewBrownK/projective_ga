use std::fmt;
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::format::{Format, Writer};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

pub struct DebuggableCopyPasta { pub default_formatter: Format }

impl DebuggableCopyPasta {
    pub fn new() -> Self {
        DebuggableCopyPasta { default_formatter: Format::default() }
    }
}

// Custom visitor to extract the field value
struct CopyPastaVisitor<'a> {
    value: &'a mut String,
}

impl<'a> tracing::field::Visit for CopyPastaVisitor<'a> {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "debuggable_copy_pasta" {
            *self.value = value.to_string();
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        if field.name() == "debuggable_copy_pasta" {
            // Fallback for non-string types, though we expect a String
            *self.value = format!("{:?}", value);
        }
    }
}

impl<S, N> FormatEvent<S, N> for DebuggableCopyPasta
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(&self, ctx: &FmtContext<'_, S, N>, mut writer: Writer<'_>, event: &Event<'_>) -> fmt::Result {// String to store the captured value of "debuggable_copy_pasta"
        let mut copy_pasta_value = String::new();

        // Apply the visitor to the event's fields
        let mut visitor = CopyPastaVisitor {
            value: &mut copy_pasta_value,
        };
        event.record(&mut visitor);

        if !copy_pasta_value.is_empty() {
            // If we found "debuggable_copy_pasta", write its value directly
            write!(writer, "{}", copy_pasta_value)?;
            writeln!(writer)?; // Add a newline
        } else {
            // Otherwise, use the default formatter (includes metadata)
            self.default_formatter.format_event(ctx, writer, event)?;
        }

        Ok(())
    }
}
