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
pub struct DeserializeError<'input, 'shape> {
    origin: Option<(&'input [u8], &'shape Shape<'shape>)>,
    error: (&'input [u8], &'shape Shape<'shape>),

    state: Vec<StepType<'shape>>,
    reason: ErrorReason,
}

impl<'input, 'shape> DeserializeError<'input, 'shape> {
    /// Create a new [`DeserializeError`] from
    /// the input and shape that caused the error.
    #[must_use]
    pub const fn new(
        input: &'input [u8],
        shape: &'shape Shape<'shape>,
        reason: ErrorReason,
    ) -> Self {
        Self { origin: None, error: (input, shape), state: Vec::new(), reason }
    }

    /// Add the original input and shape to the error.
    #[must_use]
    pub fn with_origin(self, input: &'input [u8], shape: &'shape Shape<'shape>) -> Self {
        Self { origin: Some((input, shape)), ..self }
    }

    /// Add the current deserializer state to the error.
    #[must_use]
    pub fn with_state(mut self, state: Vec<StepType<'shape>>) -> Self {
        self.state = state;
        self
    }

    /// Get the identifier of the shape that caused the error.
    #[must_use]
    pub const fn identifier(&self) -> &'shape str {
        match self.origin {
            Some((_, shape)) => shape.type_identifier,
            None => self.error.1.type_identifier,
        }
    }

    /// Get the [`ErrorReason`] for the error.
    #[must_use]
    pub const fn reason(&self) -> &ErrorReason { &self.reason }
}

impl Error for DeserializeError<'_, '_> {}

// -------------------------------------------------------------------------------------------------

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

    /// Get a detailed error message for the error.
    #[must_use]
    pub fn error_message(&self, shape: &Shape<'_>) -> String {
        match self {
            ErrorReason::EndOfInput => {
                String::from("The input ended unexpectedly while parsing data.")
            }
            ErrorReason::InvalidBool(byte) => {
                format!("Expected either `true` (1) or `false` (0), but found `{byte}`.")
            }
            // TODO: Figure out how to get multi-line error messages working.
            ErrorReason::InvalidVariant(var) => {
                let mut message = format!("Invalid enum variant `{var}`, expected one of: ");
                if let Type::User(UserType::Enum(ty)) = shape.ty {
                    for (index, variant) in ty.variants.iter().enumerate() {
                        write!(
                            message,
                            "{} ({})",
                            variant.name,
                            variant.discriminant.unwrap_or_default()
                        )
                        .unwrap();
                        if index < ty.variants.len() - 1 {
                            message.push_str(", ");
                        }
                    }
                }

                message
            }
            ErrorReason::InvalidUtf8(..) => {
                String::from("Strings must be valid UTF-8, but the input contained invalid bytes.")
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(not(feature = "rich-diagnostics"))]
impl Debug for DeserializeError<'_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.reason.error_message(self.error.1))
    }
}

#[cfg(not(feature = "rich-diagnostics"))]
impl Display for DeserializeError<'_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.reason.error_message(self.error.1))
    }
}

impl DeserializeError<'_, '_> {}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "rich-diagnostics")]
impl Debug for DeserializeError<'_, '_> {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.eprintln();
        Ok(())
    }
}

#[cfg(feature = "rich-diagnostics")]
impl Display for DeserializeError<'_, '_> {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.eprintln();
        Ok(())
    }
}

#[cfg(feature = "rich-diagnostics")]
impl DeserializeError<'_, '_> {
    /// Create and print a [`Report`] to `stderr`.
    pub fn eprintln(&self) {
        let (source, span) = self.build_source();

        let mut builder =
            Report::build(ReportKind::Error, (self.identifier().to_string(), 0..source.len()));

        // Add the error label and message.
        builder = builder.with_label(self.build_reason(span));
        builder = builder.with_message(self.reason.error_reason());

        // Finish and print the report.
        let _ = builder.finish().eprint((self.identifier().to_string(), source));
    }

    fn build_reason(&self, span: Range<usize>) -> Label<(String, Range<usize>)> {
        Label::new((self.identifier().to_string(), span))
            .with_message(self.reason.error_message(self.error.1))
            .with_order(-128)
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
    #[must_use]
    fn build_source(&self) -> (Source, Range<usize>) {
        let mut source = String::new();
        let mut span = 0..0;

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

                // Get the start and end of the span to print.
                let error_pos = input.len().saturating_sub(self.error.0.len());
                let (start, end) = (error_pos.saturating_sub(4), (error_pos + 4).min(input.len()));

                source.push('[');
                // Add an ellipsis if the array is truncated.
                if start != 0 {
                    source.push_str("..., ");
                }

                #[expect(clippy::needless_range_loop)]
                for index in start..end {
                    // Set the span to the error position.
                    let value = input[index].to_string();
                    if index == error_pos {
                        span = source.len()..source.len() + value.len();
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

            // Build the structure leading to the error.
            source.push_str(&self.build_structure());
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
                // Set the span to the first byte of the error.
                let first = self.error.0.first().copied().unwrap_or_default().to_string();
                span = 8..8 + first.len();
            }
        }

        (Source::from(source), span)
    }

    #[expect(clippy::unused_self)]
    fn build_structure(&self) -> String { String::new() }
}
