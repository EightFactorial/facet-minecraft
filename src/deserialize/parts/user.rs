use facet::{EnumType, StructType, UserType};
use facet_reflect::Partial;

use crate::{
    DeserializeError, DeserializerExt,
    deserialize::{DeserializerState, StepType, error::ErrorReason},
};

pub(crate) fn deserialize_user<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    ty: UserType<'shape>,
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'input, 'shape>,
    de: &mut D,
) -> Result<(&'partial mut Partial<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'shape>>
{
    match ty {
        UserType::Struct(ty) => deserialize_struct(ty, current, input, state, de),
        UserType::Enum(ty) => deserialize_enum(ty, current, input, state, de),
        UserType::Union(..) => todo!("Unsupported Union type, {:?}", current.shape()),
        UserType::Opaque => todo!("Unsupported Opaque type, {:?}", current.shape()),
    }
}

// -------------------------------------------------------------------------------------------------

fn deserialize_struct<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    ty: StructType<'shape>,
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'input, 'shape>,
    _de: &mut D,
) -> Result<(&'partial mut Partial<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'shape>>
{
    state.steps.push(StepType::Struct(ty, 0));

    if ty.fields.is_empty() {
        // Unit struct, return immediately.
        state.update_state(current, input)
    } else {
        // Begin the first field in the struct.
        let field = current.begin_nth_field(0).map_err(|err| state.handle_reflect_error(err))?;

        // Update the flags for the field.
        state.update_flags(&ty.fields[0]);

        Ok((field, input))
    }
}

fn deserialize_enum<'input, 'partial, 'facet, 'shape, D: DeserializerExt>(
    ty: EnumType<'shape>,
    current: &'partial mut Partial<'facet, 'shape>,
    input: &'input [u8],
    state: &mut DeserializerState<'input, 'shape>,
    de: &mut D,
) -> Result<(&'partial mut Partial<'facet, 'shape>, &'input [u8]), DeserializeError<'input, 'shape>>
{
    // Read the variant discriminant from the input.
    let (variant_disc, remaining) =
        de.deserialize_var_i64(input).map_err(|err| state.handle_deserialize_error(err))?;

    // Get the variant index from the discriminant.
    let Some(variant_indx) =
        ty.variants.iter().position(|v| v.discriminant.unwrap_or_default() == variant_disc)
    else {
        return Err(state.handle_deserialize_error(
            DeserializeError::new(
                input,
                current.shape(),
                ErrorReason::InvalidVariant(variant_disc),
            )
            .with_length(input.len() - remaining.len()),
        ));
    };

    let ty_variant = &ty.variants[variant_indx];
    state.steps.push(StepType::Enum(ty_variant, 0));

    // Start the enum variant.
    let variant =
        current.select_nth_variant(variant_indx).map_err(|err| state.handle_reflect_error(err))?;

    if ty_variant.data.fields.is_empty() {
        // Unit struct, return immediately.
        state.update_state(variant, remaining)
    } else {
        // Begin the first field in the enum.
        let field =
            variant.begin_nth_enum_field(0).map_err(|err| state.handle_reflect_error(err))?;

        // Update the flags for the field.
        state.update_flags(&ty_variant.data.fields[0]);

        Ok((field, remaining))
    }
}
