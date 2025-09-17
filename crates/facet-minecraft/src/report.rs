//! A [`Report`] generator and display wrapper.

use alloc::{borrow::Cow, format, string::String, vec::Vec};
use core::{
    fmt::{Debug, Display, Write},
    ops::Range,
};

use ariadne::{Label, Report, ReportKind, Source};

use crate::{
    deserialize::DeserError,
    serialize::{SerError, Writer},
};

/// A labeled range in the report.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabeledRange<'a> {
    /// The message associated with the range.
    pub message: Cow<'a, str>,
    /// The range in the report's data.
    pub range: Range<usize>,
}

impl<'a> LabeledRange<'a> {
    /// Create a new [`LabeledRange`].
    #[inline]
    #[must_use]
    pub fn new<T: Into<Cow<'a, str>>>(message: T, range: Range<usize>) -> Self {
        Self { message: message.into(), range }
    }

    /// Take ownership of the range, copying the message if necessary.
    #[must_use]
    pub fn into_owned(self) -> LabeledRange<'static> {
        LabeledRange { message: Cow::Owned(self.message.into_owned()), range: self.range }
    }
}

// -------------------------------------------------------------------------------------------------

/// A [`Report`] with a source for display.
pub struct ReportDisplay<'a, T: ariadne::Span> {
    report: Report<'a, T>,
    display: Source<Cow<'a, str>>,
    location: &'static str,
}

impl<T: ariadne::Span> ReportDisplay<'_, T> {
    /// Get the underlying [`ariadne::Report`].
    #[inline]
    #[must_use]
    pub const fn report(&self) -> &Report<'_, T> { &self.report }
}

#[expect(clippy::missing_errors_doc, reason = "Doesn't need it.")]
impl<T: ariadne::Span<SourceId = &'static str>> ReportDisplay<'_, T> {
    /// Write this diagnostic to an implementor of [`Write`](std::io::Write).
    ///
    /// See [`Report::write`] for more details.
    pub fn write<W: std::io::Write>(&self, w: W) -> std::io::Result<()> {
        self.report.write::<_, W>((self.location, &self.display), w)
    }

    /// Write this diagnostic to an implementor of [`Write`](std::io::Write),
    /// assuming that the output is ultimately going to be printed to
    /// `stdout`.
    ///
    /// See [`Report::write_for_stdout`] for more details.
    pub fn write_for_stdout<W: std::io::Write>(&self, w: W) -> std::io::Result<()> {
        self.report.write_for_stdout((self.location, &self.display), w)
    }

    /// Write this diagnostic out to `stderr`.
    ///
    /// See [`Report::eprint`] for more details.
    pub fn eprint(&self) -> std::io::Result<()> {
        self.report.eprint((self.location, &self.display))
    }

    /// Write this diagnostic out to `stdout`.
    ///
    /// See [`Report::print`] for more details.
    pub fn print(&self) -> std::io::Result<()> { self.report.print((self.location, &self.display)) }
}

impl<T: ariadne::Span<SourceId = &'static str>> Debug for ReportDisplay<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}
impl<T: ariadne::Span<SourceId = &'static str>> Display for ReportDisplay<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut buffer = Vec::with_capacity(self.display.text().len());
        self.write_for_stdout(&mut buffer).map_err(|_| core::fmt::Error)?;
        f.write_str(core::str::from_utf8(&buffer).map_err(|_| core::fmt::Error)?)
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> From<&DeserError<'a>> for ReportDisplay<'a, (&'static str, Range<usize>)> {
    fn from(err: &DeserError<'a>) -> Self {
        // Create the display string and span map.
        let mut display = String::with_capacity(err.source().len() * 3 + 2);
        let mut span_map = Vec::with_capacity(err.source().len());
        let location = err.location().map_or("<unknown>", |loc| loc.file());

        {
            display.push('[');
            for (i, byte) in err.source().iter().enumerate() {
                let start = display.len();
                write!(display, "{byte}").unwrap();
                span_map.push(start..display.len());
                if i < err.source().len() - 1 {
                    display.push_str(", ");
                }
            }
            display.push(']');
        }

        // Create a span that covers all other spans.
        let mut start = err.span().start;
        let mut end = err.span().end;
        for label in err.labels() {
            start = start.min(label.range.start);
            end = end.max(label.range.end);
        }

        // Build the report using spans mapped to the source string.
        let mut builder =
            Report::build(ReportKind::Error, map_span(&(start..end), location, &span_map));
        // Set the message
        builder.set_message(format!(
            "Failed to read `{}`: {}",
            err.type_name(),
            err.kind().description()
        ));

        // Add the main error label
        builder.add_label(
            Label::new(map_span(err.span(), location, &span_map))
                .with_message(err.kind().as_message())
                .with_order(-1024),
        );

        // Add additional labels in order
        builder.add_labels(err.labels().iter().enumerate().map(|(i, label)| {
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_possible_wrap,
                reason = "There will never be that many labels"
            )]
            Label::new(map_span(&label.range, location, &span_map))
                .with_message(&label.message)
                .with_order(i as i32)
        }));

        // Add any help messages and notes
        for (message, kind) in err.messages() {
            if *kind {
                builder.add_help(message);
            } else {
                builder.add_note(message);
            }
        }

        ReportDisplay {
            location,
            report: builder.finish(),
            display: Source::from(Cow::Owned(display)),
        }
    }
}

/// Map a range to the display string using the provided ranges as a map.
fn map_span<T>(span: &Range<usize>, id: T, span_map: &[Range<usize>]) -> (T, Range<usize>) {
    let start = span_map.get(span.start).map_or(0, |r| r.start);
    let end = span_map.get(span.end).map(|r| r.end);
    let end = end.unwrap_or_else(|| span_map.last().map_or(0, |r| r.end));
    (id, start..end)
}

// -------------------------------------------------------------------------------------------------

impl<'a, W: Writer> From<&SerError<'a, W>> for ReportDisplay<'a, (&'static str, Range<usize>)> {
    fn from(_value: &SerError<'a, W>) -> Self { todo!() }
}
