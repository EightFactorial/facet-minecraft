//! A trait and methods for asserting that a type can be read and written.
#![expect(clippy::single_match)]

#[cfg(feature = "custom")]
use facet::ShapeAttribute;
use facet::{
    Def, EnumType, Facet, Field, FieldAttribute, NumericType, PointerType, PrimitiveType,
    SequenceType, Shape, ShapeLayout, StructType, TextualType, Type, UserType,
};

/// A trait for asserting that a type can be read and written.
pub trait AssertProtocol<'a>: Facet<'a> {
    /// An assertion that the type can be read and written.
    const ASSERT: () = assert!(valid_shape(Self::SHAPE), "Type cannot be read/written!");
    /// An assertion that the type can be read and written.
    fn assert() { const { Self::ASSERT } }
}

impl<'a, T: Facet<'a>> AssertProtocol<'a> for T {}

// -------------------------------------------------------------------------------------------------

/// Returns `true` if the given [`Shape`] can be read and written.
#[must_use]
const fn valid_shape(shape: &Shape<'_>) -> bool {
    #[cfg(feature = "custom")]
    {
        let mut index = 0usize;
        while index < shape.attributes.len() {
            if let ShapeAttribute::Arbitrary(attr) = shape.attributes[index] {
                match attr.as_bytes() {
                    // Support any type with the `custom` attribute.
                    b"custom" => return true,
                    _ => {}
                }
            }
            index += 1;
        }
    }

    match shape.ty {
        Type::Primitive(primitive) => {
            !matches!(primitive, PrimitiveType::Textual(TextualType::Char) | PrimitiveType::Never)
        }
        Type::Sequence(SequenceType::Array(array)) => valid_shape(array.t),
        Type::Sequence(SequenceType::Slice(slice)) => valid_shape(slice.t),
        Type::User(UserType::Struct(user)) => valid_struct_type(&user),
        Type::User(UserType::Enum(user)) => valid_enum_type(&user),
        Type::User(UserType::Opaque) => match shape.def {
            // TODO: `IpAddr` and `SocketAddr` are not supported.
            // Def::Scalar => todo!(),
            // TODO: Cannot determine if shapes are valid at compile time.
            // Def::Map(def) => valid_shape(def.k()) && valid_shape(def.v()),
            // Def::Set(def) => valid_shape(def.t()),
            // Def::List(def) => valid_shape(def.t()),
            // Def::SmartPointer(def) => valid_shape(def.pointee),
            Def::Array(def) => valid_shape(def.t),
            Def::Slice(def) => valid_shape(def.t),
            Def::Option(def) => valid_shape(def.t),
            Def::Undefined => false,
            _ => true,
        },
        Type::User(UserType::Union(..)) => panic!("Unions are not supported yet!"),
        // TODO: Cannot determine if shapes are valid at compile time.
        // Type::Pointer(PointerType::Reference(ptr)) => valid_shape(ptr.target()),
        Type::Pointer(PointerType::Reference(..)) => true,
        Type::Pointer(..) => panic!("Pointers are not supported yet!"),
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
        let FieldAttribute::Arbitrary(attr) = field.attributes[index];

        match attr.as_bytes() {
            // Check for the variably-sized marker attribute.
            b"var" => {
                // Make sure the type is u16/i16, u32/i32, u64/i64, u128/u128, or usize/isize.
                // Other sizes and floats are not allowed to be variably sized.
                if let Type::Primitive(PrimitiveType::Numeric(numeric)) = field.shape.ty {
                    if let ShapeLayout::Sized(layout) = field.shape().layout {
                        match numeric {
                            // Accept u16/i16, u32/i32, u64/i64, i128/u128, and usize/isize.
                            NumericType::Integer { .. }
                                if matches!(layout.size(), 2 | 4 | 8 | 16) => {}
                            // Reject u8/i8
                            NumericType::Integer { .. } => {
                                panic!("u8/i8 cannot be variably sized!")
                            }
                            // Reject f16, f32, f64, and f128
                            NumericType::Float => {
                                panic!("Floating point types cannot be variably sized!")
                            }
                        }
                    } else {
                        panic!("Only numeric types (u16, u32, i64, etc) can be variably sized!");
                    }
                }
            }
            _ => {}
        }

        index += 1;
    }
    valid_shape(field.shape)
}
