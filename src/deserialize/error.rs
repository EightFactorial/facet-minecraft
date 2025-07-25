#[cfg(feature = "rich-diagnostics")]
use alloc::string::ToString;
use alloc::{format, string::String, vec::Vec};
#[cfg(feature = "rich-diagnostics")]
use core::ops::Range;
use core::{
    error::Error,
    fmt::{Debug, Display, Write},
};

#[cfg(feature = "rich-diagnostics")]
use ariadne::{Label, Report, ReportKind, Source};
use facet::{Shape, Type, UserType};

use crate::deserialize::StepType;

/// An error that occurred during deserialization.
pub struct DeserializeError<'input> {
    origin: Option<(&'input [u8], &'static Shape)>,
    error: (&'input [u8], &'static Shape),
    #[allow(dead_code)]
    length: Option<usize>,

    state: Vec<StepType>,
    reason: ErrorReason,
}

impl<'input> DeserializeError<'input> {
    /// Create a new [`DeserializeError`] from
    /// the input and shape that caused the error.
    #[must_use]
    pub const fn new(input: &'input [u8], shape: &'static Shape, reason: ErrorReason) -> Self {
        Self { origin: None, error: (input, shape), length: None, state: Vec::new(), reason }
    }

    /// Set the number of bytes that caused the error.
    #[must_use]
    pub fn with_length(self, length: usize) -> Self { Self { length: Some(length), ..self } }

    /// Add the original input and shape to the error.
    #[must_use]
    pub fn with_origin(self, input: &'input [u8], shape: &'static Shape) -> Self {
        Self { origin: Some((input, shape)), ..self }
    }

    /// Add the current deserializer state to the error.
    #[must_use]
    pub fn with_state(mut self, state: Vec<StepType>) -> Self {
        self.state = state;
        self
    }

    /// Get the identifier of the shape that caused the error.
    #[must_use]
    pub const fn identifier(&self) -> &'static str {
        match self.origin {
            Some((_, shape)) => shape.type_identifier,
            None => self.error.1.type_identifier,
        }
    }

    /// Get the [`ErrorReason`] for the error.
    #[must_use]
    pub const fn reason(&self) -> &ErrorReason { &self.reason }
}

impl Error for DeserializeError<'_> {}

// -------------------------------------------------------------------------------------------------

/// A reason for a [`DeserializeError`].
#[non_exhaustive]
#[expect(missing_docs)]
pub enum ErrorReason {
    EndOfInput,
    InvalidBool(u8),
    InvalidVariant(i64),
    InvalidUtf8(usize),
}

impl ErrorReason {
    /// Get a short description of the error.
    #[must_use]
    pub const fn error_reason(&self) -> &'static str {
        match self {
            ErrorReason::EndOfInput => "Unexpected end of input",
            ErrorReason::InvalidBool(..) => "Invalid boolean value",
            ErrorReason::InvalidVariant(..) => "Invalid enum variant",
            ErrorReason::InvalidUtf8(..) => "Invalid UTF-8 string",
        }
    }

    /// Get a label describing what caused the error.
    #[must_use]
    pub fn error_label(&self) -> String {
        match self {
            ErrorReason::EndOfInput => {
                String::from("The input ended unexpectedly while parsing data.")
            }
            ErrorReason::InvalidBool(byte) => {
                format!("Expected either `true` (1) or `false` (0), but found `{byte}`.")
            }
            ErrorReason::InvalidVariant(var) => format!("Invalid enum variant `{var}`"),
            ErrorReason::InvalidUtf8(..) => {
                String::from("Strings must be valid UTF-8, but the input contained invalid bytes.")
            }
        }
    }

    /// Get a note describing what was expected.
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn expected_note(&self, error: &DeserializeError<'_>) -> Option<String> {
        match self {
            ErrorReason::EndOfInput => {
                if let Some((input, _)) = error.origin
                    && let Some(length) = error.length
                {
                    let start = input.len().saturating_sub(error.error.0.len());
                    match length - (input.len() - start) {
                        0 => None,
                        1 => Some(format!(
                            "Expected 1 more byte for a `{}`",
                            error.error.1.type_identifier
                        )),
                        other => Some(format!(
                            "Expected {other} more bytes for a `{}`",
                            error.error.1.type_identifier
                        )),
                    }
                } else {
                    None
                }
            }
            ErrorReason::InvalidVariant(_) => {
                if let Type::User(UserType::Enum(ty)) = error.error.1.ty {
                    let mut message = String::from("Expected one of:\n");

                    // Add each enum variant to the message.
                    for (index, variant) in ty.variants.iter().enumerate() {
                        write!(
                            message,
                            "{}::{} ({})",
                            error.error.1.type_identifier,
                            variant.name,
                            variant.discriminant.unwrap_or_default()
                        )
                        .unwrap();
                        if index < ty.variants.len().saturating_sub(1) {
                            message.push('\n');
                        }
                    }

                    Some(message)
                } else {
                    None
                }
            }
            ErrorReason::InvalidUtf8(pos) => {
                let slice = str::from_utf8(&error.error.0[..*pos]).unwrap();
                Some(format!("Valid string slice: \"{slice}\" (0..{pos})"))
            }
            _ => None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(not(feature = "rich-diagnostics"))]
impl Debug for DeserializeError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.reason.error_label())
    }
}

#[cfg(not(feature = "rich-diagnostics"))]
impl Display for DeserializeError<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.reason.error_label())
    }
}

impl DeserializeError<'_> {}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "rich-diagnostics")]
impl Debug for DeserializeError<'_> {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.eprint();
        Ok(())
    }
}

#[cfg(feature = "rich-diagnostics")]
impl Display for DeserializeError<'_> {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.eprint();
        Ok(())
    }
}

#[cfg(feature = "rich-diagnostics")]
impl DeserializeError<'_> {
    /// Print the error report to stdout.
    ///
    /// In most cases, [`DeserializeError::eprint`] is the
    /// ['more correct'](https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)) function to use.
    #[expect(clippy::doc_link_with_quotes)]
    pub fn print(&self) {
        let (report, source) = self.report();
        let _ = report.print((self.identifier().to_string(), source));
    }

    /// Print the error report to stderr.
    pub fn eprint(&self) {
        let (report, source) = self.report();
        let _ = report.eprint((self.identifier().to_string(), source));
    }

    /// Write the error report to the given writer.
    ///
    /// If you are writing to `stdout` or `stderr`, consider using
    /// [`DeserializeError::print`] or [`DeserializeError::eprint`] instead.
    ///
    /// # Errors
    /// Returns an error if the writer fails to write the report.
    pub fn write(&self, writer: impl std::io::Write) -> std::io::Result<()> {
        let (report, source) = self.report();
        report.write((self.identifier().to_string(), source), writer)
    }

    /// Build a [`Report`] for the error.
    #[must_use]
    pub fn report(&self) -> (Report<'_, (String, Range<usize>)>, Source) {
        let (mut source, error_span) = self.build_source();
        let structure_span = self.build_structure(&mut source);

        let mut builder =
            Report::build(ReportKind::Error, (self.identifier().to_string(), error_span.clone()));

        // Add the error message and label.
        builder = builder.with_message(self.reason.error_reason());
        builder = builder.with_label(self.reason_label(error_span));

        // If there is a structure label, add it.
        if let Some(span) = structure_span {
            builder = builder.with_label(self.structure_label(span));
        }

        // If the error has a note, add it.
        if let Some(note) = self.reason.expected_note(self) {
            builder = builder.with_note(note);
        }

        // Finish and print the report.
        (builder.finish(), Source::from(source))
    }

    fn reason_label(&self, span: Range<usize>) -> Label<(String, Range<usize>)> {
        Label::new((self.identifier().to_string(), span))
            .with_message(self.reason.error_label())
            .with_order(-128)
    }

    fn structure_label(&self, span: Range<usize>) -> Label<(String, Range<usize>)> {
        Label::new((self.identifier().to_string(), span))
            .with_message("Error occurred reading this field")
    }

    /// Build a [`Source`] for the error to use.
    ///
    /// This will provide content for the error report to use.
    ///
    /// # Example:
    ///
    /// ```text
    /// # If no origin is provided:
    /// Input: [0, 1, 2, 3, 4, 5, 6, 7, ...]
    ///         _
    ///         |---> Reason For Error
    ///
    /// # If the origin is provided:
    /// Input: [..., 4, 5, 6, 7, 8, 9, 10, 11, ...]
    ///         ___
    ///          |---> Reason For Error
    /// ```
    #[must_use]
    fn build_source(&self) -> (String, Range<usize>) {
        let mut source = String::new();
        let mut span = 8..8;

        if let Some((input, ..)) = self.origin {
            // Print the input that caused the error.
            source.push_str("Input: ");

            if input.is_empty() {
                // If the input is empty, print "<empty>".
                // Ex: "Input: <empty>"
                source.push_str("<empty>");
            } else {
                // Print 8 bytes around the error, truncating the ends if necessary.
                // Ex: "Input: [..., 4, 5, 6, 7, 8, 9, 10, 11, ...]"

                // Get the position of the error in the input.
                let mut error_pos = match self.reason {
                    ErrorReason::InvalidUtf8(pos) => {
                        input.len().saturating_sub(self.error.0.len()) + pos
                    }
                    _ => input.len().saturating_sub(self.error.0.len()),
                };
                // Never point past the end of the input.
                if error_pos == input.len() {
                    error_pos = input.len().saturating_sub(1);
                }

                // Get the start and end of the span to print.
                let (start, mut end) =
                    (error_pos.saturating_sub(4), (error_pos + 4).min(input.len()));
                if end - start < 8 {
                    end = (end + 8 - (end - start)).min(input.len());
                }

                source.push('[');
                // Add an ellipsis if the array is truncated.
                if start != 0 {
                    source.push_str("..., ");
                }

                #[expect(clippy::needless_range_loop)]
                for index in start..end {
                    // Set the span to the error position.
                    let value = input[index].to_string();

                    if let Some(length) = self.length
                        && span.start != 8
                    {
                        // Grow the span to include the new value.
                        if (error_pos..(error_pos + length)).contains(&index) {
                            span = span.start..span.end + 2 + value.len();
                        }
                    } else {
                        // Set the span to the error position.
                        if index == error_pos {
                            span = source.len()..source.len() + value.len();
                        }
                    }

                    // Push the value to the source.
                    source.push_str(&value);
                    if index < end - 1 {
                        source.push_str(", ");
                    }
                }

                // Add an ellipsis if the array is truncated.
                if end != input.len() {
                    source.push_str(", ...");
                }
                source.push(']');
            }
        } else {
            // Print the input that caused the error.
            source.push_str("Input: ");

            match self.error.0.len() {
                // If the input is empty, print "<empty>".
                // Ex: "Input: <empty>"
                0 => source.push_str("<empty>"),
                // Print the first 8 bytes of the error.
                // Ex: "Input: [0, 1, 2, 3, 4, 5, 6, 7]"
                1..9 => write!(source, "{:?}", self.error.0).unwrap(),
                // Truncate anything longer than 8 bytes.
                // Ex: "Input: [0, 1, 2, 3, 4, 5, 6, 7, ...]"
                9.. => {
                    let slice = format!("{:?}", &self.error.0[..8]);
                    source.push_str(&slice[..slice.len().saturating_sub(1)]);
                    source.push_str(", ...]");
                }
            }

            if !self.error.0.is_empty() {
                let take = self.length.unwrap_or_default().max(1);
                for byte in self.error.0.iter().take(take) {
                    span = span.start..span.end + byte.to_string().len();
                }
            }
        }

        (source, span)
    }

    /// Build a [`Source`] for the error to use.
    ///
    /// This will provide content for the error report to use.
    ///
    /// # Example:
    ///
    /// ```text
    /// # If no origin is provided:
    ///
    /// # If the origin is provided:
    ///
    /// MyStruct {
    ///     ... # Other fields
    ///     relevant_field: FieldType {
    ///         previous_field: u32,
    ///         current_field: u32,
    ///         ___________
    ///              |---> Error Occurred Here
    ///         future_field: u32,
    ///         }
    ///     ... # Other fields
    /// }
    /// ```
    #[expect(clippy::ptr_arg, clippy::unused_self)]
    fn build_structure(&self, _source: &mut String) -> Option<Range<usize>> { None }
}
