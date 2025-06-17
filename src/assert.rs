//! A trait and methods for asserting that a type can be read and written.

use facet::{
    EnumType, Facet, Field, FieldAttribute, NumericType, PrimitiveType, SequenceType, Shape,
    ShapeLayout, StructType, Type, UserType,
};

/// A trait for asserting that a type can be read and written.
pub trait AssertProtocol<'a>: Facet<'a> {
    /// An assertion that the type can be read and written.
    const ASSERT: () = assert!(valid_shape(Self::SHAPE), "Type cannot be read/written!");
    /// An assertion that the type can be read and written.
    fn assert() { let () = Self::ASSERT; }
}

impl<'a, T: Facet<'a>> AssertProtocol<'a> for T {}

// -------------------------------------------------------------------------------------------------

/// Returns `true` if the given [`Shape`] can be read and written.
#[must_use]
pub const fn valid_shape(shape: &Shape<'_>) -> bool {
    match shape.ty {
        Type::Primitive(primitive) => !matches!(primitive, PrimitiveType::Never),
        Type::Sequence(SequenceType::Array(array)) => valid_shape(array.t),
        Type::Sequence(SequenceType::Slice(slice)) => valid_shape(slice.t),
        Type::User(UserType::Struct(user)) => valid_struct_type(&user),
        Type::User(UserType::Enum(user)) => valid_enum_type(&user),
        Type::User(UserType::Union(..) | UserType::Opaque) => false,
        Type::Pointer(..) => panic!("Pointer types are not supported yet!"),
        _ => panic!("This type does not support assertions yet!"),
    }
}

#[must_use]
#[rustfmt::skip]
const fn valid_struct_type(ty: &StructType<'_>) -> bool {
    let mut index = 0usize;
    while index < ty.fields.len() {
        if !valid_field(&ty.fields[index]) { return false }
        index += 1;
    }
    true
}

#[must_use]
#[rustfmt::skip]
const fn valid_enum_type(ty: &EnumType<'_>) -> bool {
    let mut index = 0usize;
    while index < ty.variants.len() {
        let field = &ty.variants[index];
        if !valid_struct_type(&field.data) { return false }
        index += 1;
    }
    true
}

/// Returns `true` if the given [`Field`] can be read and written.
#[must_use]
const fn valid_field(field: &Field<'_>) -> bool {
    let mut index = 0usize;
    while index < field.attributes.len() {
        if let FieldAttribute::Arbitrary(attr) = field.attributes[index] {
            match attr.as_bytes() {
                // Check for the variably-sized marker attribute.
                b"frog(var)" | b"frog(variable)" => {
                    // Make sure the type is u16/i16, u32/i32, or u64/i64.
                    // Other sizes and floats are not allowed to be variably sized.
                    if let Type::Primitive(PrimitiveType::Numeric(numeric)) = field.shape.ty {
                        if let ShapeLayout::Sized(layout) = field.shape().layout {
                            match numeric {
                                // Accept u16/i16, u32/i32, and u64/i64
                                NumericType::Integer { .. }
                                    if matches!(layout.size(), 2 | 4 | 8) => {}
                                // Reject u8/i8 and u128/i128
                                NumericType::Integer { .. } => {
                                    panic!("u8/i8 and u128/i128 cannot be variably sized!")
                                }
                                // Reject f16, f32, f64, and f128
                                NumericType::Float => {
                                    panic!("Floating point types cannot be variably sized!")
                                }
                            }
                        } else {
                            panic!(
                                "Only numeric types (u16, u32, i64, etc) can be variably sized!"
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        index += 1;
    }

    valid_shape(field.shape)
}
