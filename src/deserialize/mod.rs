use alloc::vec::Vec;

#[cfg(feature = "custom")]
use facet::ShapeAttribute;
use facet::{
    ArrayType, Def, Field, FieldAttribute, SequenceType, Shape, SliceType, StructType, Type,
    Variant,
};
use facet_reflect::{HeapValue, Partial, ReflectError};

use crate::assert::AssertProtocol;
#[cfg(feature = "custom")]
use crate::custom::FacetOverride;

mod error;
pub use error::DeserializeError;

mod parts;
use parts::{deserialize_json, deserialize_primitive, deserialize_sequence, deserialize_user};

mod traits;
pub use traits::{Deserializer, DeserializerExt};

/// Deserialize a type from the given byte slice.
///
/// This is a wrapper around [`deserialize_iterative`],
/// using [`McDeserializer`] as the deserializer.
///
/// # Errors
/// Returns an error if the deserialization fails.
#[inline(always)]
#[expect(clippy::inline_always)]
pub fn deserialize<'input: 'facet, 'facet, 'shape, T: AssertProtocol<'facet>>(
    input: &'input [u8],
) -> Result<(T, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
    McDeserializer::deserialize::<T>(input)
}

// -------------------------------------------------------------------------------------------------

/// A deserializer for Minecraft protocol data.
#[derive(Debug, Default, Clone, Copy)]
pub struct McDeserializer;

impl McDeserializer {
    /// Deserialize a type from the given byte slice.
    ///
    /// This is a wrapper around [`deserialize_iterative`],
    /// using [`McDeserializer`] as the deserializer.
    ///
    /// # Errors
    /// Returns an error if the deserialization fails.
    #[inline(always)]
    #[expect(clippy::inline_always)]
    pub fn deserialize<'input: 'facet, 'facet, 'shape, T: AssertProtocol<'facet>>(
        input: &'input [u8],
    ) -> Result<(T, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
        let () = const { <T as AssertProtocol<'facet>>::ASSERT };

        deserialize_iterative::<T, McDeserializer>(input, T::SHAPE, McDeserializer)
    }
}

// -------------------------------------------------------------------------------------------------

/// Iteratively deserialize a type from the given bytes.
///
/// Avoids recursion to prevent depth issues with large structures.
///
/// # Errors
/// Returns an error if the deserialization fails.
pub fn deserialize_iterative<
    'input: 'facet,
    'facet,
    'shape,
    T: AssertProtocol<'facet>,
    D: DeserializerExt,
>(
    input: &'input [u8],
    shape: &'shape Shape<'shape>,
    mut de: D,
) -> Result<(T, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
    let partial = match Partial::alloc_shape(shape) {
        Ok(partial) => partial,
        Err(_err) => todo!(),
    };

    let (heap, rem) = match deserialize_value::<D>(input, partial, &mut de) {
        Ok(value) => value,
        Err(_err) => todo!(),
    };

    match heap.materialize::<T>() {
        Ok(value) => Ok((value, rem)),
        Err(_err) => todo!(),
    }
}

fn deserialize_value<'input: 'facet, 'facet, 'shape, D: DeserializerExt>(
    mut input: &'input [u8],
    mut partial: Partial<'facet, 'shape>,
    de: &mut D,
) -> Result<(HeapValue<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'facet, 'shape>> {
    #[cfg(feature = "custom")]
    let overrides = FacetOverride::global();

    let mut current = &mut partial;
    let mut state = DeserializerState::default();

    loop {
        // Use the inner type if the shape has the `transparent` attribute.
        if current.shape().attributes.contains(&ShapeAttribute::Transparent) {
            current = current.begin_inner().unwrap();
        }

        // If the shape has a `custom` attribute,
        // check for a custom deserialization function.
        #[cfg(feature = "custom")]
        {
            static CUSTOM: &ShapeAttribute = &ShapeAttribute::Arbitrary("custom");

            if current.shape().attributes.contains(CUSTOM)
                && let Some(custom) = overrides.iter().find(|o| o.id == current.shape().id)
                && let Some(de) = custom.deserialize
            {
                // Call the custom deserializer and update the state.
                let (partial, remaining) =
                    de(current, input).map_err(|err| state.handle_deserialize_error(err))?;
                let (partial, remaining) = state.update_state(partial, remaining)?;

                // Re-assign the current partial and consume the input.
                current = partial;
                input = remaining;

                // If we've finished the last frame, break the loop.
                if current.frame_count() == 1 {
                    break;
                }

                // Otherwise, continue to the next frame.
                continue;
            }
        }

        // If the shape has a `json` attribute,
        // deserialize the value as JSON.
        #[cfg(feature = "json")]
        if state.json() {
            // Deserialize the value as JSON.
            let (partial, remaining) = deserialize_json(current, input, &mut state, de)
                .map_err(|err| state.handle_deserialize_error(err))?;

            // Re-assign the current partial and consume the input.
            current = partial;
            input = remaining;

            // If we've finished the last frame, break the loop.
            if current.frame_count() == 1 {
                break;
            }

            // Otherwise, continue to the next frame.
            continue;
        }

        // Deserialize the value
        match current.shape().def {
            Def::Scalar | Def::Undefined => {
                match current.shape().ty {
                    Type::Primitive(..) => {
                        let (partial, remaining) =
                            deserialize_primitive(current, input, &mut state, de)?;
                        // Re-assign the current partial and consume the input.
                        current = partial;
                        input = remaining;
                    }
                    Type::Sequence(ty) => {
                        let (partial, remaining) =
                            deserialize_sequence(ty, current, input, &mut state, de)?;
                        // Re-assign the current partial and consume the input.
                        current = partial;
                        input = remaining;
                    }
                    Type::User(ty) => {
                        let (partial, remaining) =
                            deserialize_user(ty, current, input, &mut state, de)?;
                        // Re-assign the current partial and consume the input.
                        current = partial;
                        input = remaining;
                    }
                    Type::Pointer(_ty) => todo!(),
                }
            }
            Def::Array(def) => {
                let ty = SequenceType::Array(ArrayType { t: def.t, n: def.n });
                let (partial, remaining) =
                    deserialize_sequence(ty, current, input, &mut state, de)?;
                // Re-assign the current partial and consume the input.
                current = partial;
                input = remaining;
            }
            Def::List(def) => {
                let ty = SequenceType::Slice(SliceType { t: def.t() });
                let (partial, remaining) =
                    deserialize_sequence(ty, current, input, &mut state, de)?;
                // Re-assign the current partial and consume the input.
                current = partial;
                input = remaining;
            }
            Def::Slice(def) => {
                let ty = SequenceType::Slice(SliceType { t: def.t() });
                let (partial, remaining) =
                    deserialize_sequence(ty, current, input, &mut state, de)?;
                // Re-assign the current partial and consume the input.
                current = partial;
                input = remaining;
            }
            Def::Option(_def) => todo!(),
            Def::Map(_def) => todo!(),
            Def::Set(_def) => todo!(),
            Def::SmartPointer(_def) => todo!(),
        }

        // If we've finished the last frame, break the loop.
        if current.frame_count() == 1 {
            break;
        }
    }

    // Build the deserialized value.
    match partial.build() {
        Ok(heap) => Ok((heap, input)),
        Err(err) => Err(state.handle_reflect_error(err)),
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct DeserializerState<'shape> {
    steps: Vec<StepType<'shape>>,
    /// (var, json)
    flags: (bool, bool),
}

#[derive(Debug)]
enum StepType<'shape> {
    Sequence(usize, usize),
    Struct(StructType<'shape>, usize),
    Enum(&'shape Variant<'shape>, usize),
}

impl<'shape> DeserializerState<'shape> {
    /// Returns `true` if the next field is using variable-length encoding.
    #[must_use]
    const fn variable(&self) -> bool { self.flags.0 }

    /// Returns `true` if the next field is encoded as JSON.
    #[must_use]
    const fn json(&self) -> bool { self.flags.1 }

    /// Handle the result of a deserialization step.
    ///
    /// Provides better error handling, manages the current step,
    /// and updates flags for variable and JSON encoding.
    fn update_state<'input, 'partial, 'facet>(
        &mut self,
        mut partial: &'partial mut Partial<'facet, 'shape>,
        input: &'input [u8],
    ) -> Result<
        (&'partial mut Partial<'facet, 'shape>, &'input [u8]),
        DeserializeError<'input, 'facet, 'shape>,
    > {
        // If the partial has no frames left, return it.
        if partial.frame_count() == 1 {
            return Ok((partial, input));
        }

        // Otherwise, end the current step and continue the previous.
        match partial.end() {
            Ok(part) => partial = part,
            Err(err) => return Err(self.handle_reflect_error(err)),
        }

        match self.steps.last_mut() {
            Some(StepType::Sequence(length, current)) => {
                // Increment the current item index.
                *current += 1;

                if *current >= *length {
                    // Finish the sequence.
                    self.steps.pop();

                    // If the frame count is greater than 1, finish the partial.
                    if partial.frame_count() > 1 {
                        partial = partial.end().map_err(|err| self.handle_reflect_error(err))?;
                    }

                    Ok((partial, input))
                } else {
                    // Begin a new list item.
                    let list_item =
                        partial.begin_list_item().map_err(|err| self.handle_reflect_error(err))?;

                    Ok((list_item, input))
                }
            }
            Some(StepType::Struct(shape, current)) => {
                // Increment the current field index.
                *current += 1;

                if *current >= shape.fields.len() {
                    // Finish the struct.
                    self.steps.pop();

                    // If the frame count is greater than 1, finish the partial.
                    if partial.frame_count() > 1 {
                        partial = partial.end().map_err(|err| self.handle_reflect_error(err))?;
                    }

                    Ok((partial, input))
                } else {
                    let ty_field = &shape.fields[*current];

                    // Begin the next field in the struct.
                    let field = partial
                        .begin_nth_field(*current)
                        .map_err(|err| self.handle_reflect_error(err))?;

                    // Update the flags based on the current field.
                    self.update_flags(ty_field);

                    Ok((field, input))
                }
            }
            Some(StepType::Enum(variant, current)) => {
                // Increment the current field index.
                *current += 1;

                if *current >= variant.data.fields.len() {
                    // Finish the enum.
                    self.steps.pop();

                    // If the frame count is greater than 1, finish the partial.
                    if partial.frame_count() > 1 {
                        partial = partial.end().map_err(|err| self.handle_reflect_error(err))?;
                    }

                    Ok((partial, input))
                } else {
                    let ty_field = &variant.data.fields[*current];

                    // Begin the next field in the enum.
                    let field = partial
                        .begin_nth_enum_field(*current)
                        .map_err(|err| self.handle_reflect_error(err))?;

                    // Update the flags based on the current field.
                    self.update_flags(ty_field);

                    Ok((field, input))
                }
            }
            None => todo!(),
        }
    }

    /// Updates the flags based on the current step.
    fn update_flags(&mut self, field: &Field<'shape>) {
        #[cfg(feature = "json")]
        static JSON: &FieldAttribute = &FieldAttribute::Arbitrary("json");
        static VAR: &FieldAttribute = &FieldAttribute::Arbitrary("var");

        // Check if the field has a `var` attribute.
        self.flags.0 = field.attributes.contains(VAR);

        #[cfg(feature = "json")]
        {
            // Check if the field has a `json` attribute.
            self.flags.1 = field.attributes.contains(JSON);
        }
    }

    /// Populate a [`DeserializeError`] with location information.
    fn handle_deserialize_error<'input, 'facet>(
        &self,
        err: DeserializeError<'input, 'facet, 'shape>,
    ) -> DeserializeError<'input, 'facet, 'shape> {
        todo!("TODO: Handle DeserializeError: {err}")
    }

    /// Convert a [`ReflectError`] into a [`DeserializeError`]
    /// and populate it with location information.
    fn handle_reflect_error<'input, 'facet>(
        &self,
        err: ReflectError<'shape>,
    ) -> DeserializeError<'input, 'facet, 'shape> {
        todo!("TODO: Handle ReflectError: {err}")
    }
}
