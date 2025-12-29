//! Common types used for analyzing type properties at compile time.
#![allow(clippy::too_many_lines, reason = "Recursive type analysis")]

use facet::{
    Def, Field, FieldAttribute, ListDef, MapDef, NumericType, PointerType, PrimitiveType,
    SequenceType, SetDef, Shape, ShapeLayout, SliceDef, TextualType, Type, UserType,
};

/// Indicates whether a type can be serialized and whether it can fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeSerializeResult {
    /// The type will always serialize successfully.
    Infallible,
    /// The type may serialize successfully or fail.
    Fallible,
    /// The type will always fail to serialize.
    Never,
}

impl TypeSerializeResult {
    /// Returns `true` if the type will always serialize successfully.
    ///
    /// ### Note
    ///
    /// When serializing this type,
    /// the only possible errors come from I/O operations.
    #[must_use]
    pub const fn guaranteed(self) -> bool { matches!(self, TypeSerializeResult::Infallible) }

    /// Returns `true` if the type may serialize successfully.
    ///
    /// ### Note
    ///
    /// If this returns `false`,
    /// the type does not support serialization at all.
    #[must_use]
    pub const fn possible(self) -> bool { !matches!(self, TypeSerializeResult::Never) }

    /// Combines two [`TypeSerializeResult`]s into one,
    /// returning the overall result.
    #[must_use]
    pub const fn with(self, other: Self) -> Self {
        match (self, other) {
            (TypeSerializeResult::Infallible, TypeSerializeResult::Infallible) => {
                TypeSerializeResult::Infallible
            }
            (TypeSerializeResult::Never, _) | (_, TypeSerializeResult::Never) => {
                TypeSerializeResult::Never
            }
            _ => TypeSerializeResult::Fallible,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A hint for the size of a type when serialized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[expect(missing_docs, reason = "Fields don't need documentation")]
pub enum TypeSerializeHint {
    /// An exact size.
    Exact { size: usize },
    /// A size range, inclusive.
    Range { min: usize, max: Option<usize> },
    /// No size hint available.
    None,
}

impl TypeSerializeHint {
    /// Returns the minimum size hint, if available.
    #[must_use]
    pub const fn minimum(&self) -> Option<usize> {
        match self {
            TypeSerializeHint::Exact { size } => Some(*size),
            TypeSerializeHint::Range { min, .. } => Some(*min),
            TypeSerializeHint::None => None,
        }
    }

    /// Returns the maximum size hint, if available.
    #[must_use]
    pub const fn maximum(&self) -> Option<usize> {
        match self {
            TypeSerializeHint::Exact { size } => Some(*size),
            TypeSerializeHint::Range { max, .. } => *max,
            TypeSerializeHint::None => None,
        }
    }

    /// Creates a new [`TypeSerializeHint`] from the given min and max sizes.
    ///
    /// Follows the logic:
    ///   - If `min` is greater than `max`, returns `None`.
    ///   - If `min` and `max` are both `Some` and equal, returns `Exact`.
    ///   - If `min` is `Some`, returns `Range`.
    ///   - Otherwise, returns `None`.
    #[must_use]
    const fn from_min_max(min: Option<usize>, max: Option<usize>) -> Self {
        match (min, max) {
            (Some(min), Some(max)) if min > max => TypeSerializeHint::None,
            (Some(min), Some(max)) if min == max => TypeSerializeHint::Exact { size: min },
            (Some(min), max) => TypeSerializeHint::Range { min, max },
            _ => TypeSerializeHint::None,
        }
    }

    /// Multiplies the size hint by the given factor.
    ///
    /// Typically used when calculating the size of arrays.
    #[must_use]
    const fn multiply(self, n: usize) -> Self {
        match self {
            TypeSerializeHint::Exact { size } => TypeSerializeHint::Exact { size: size * n },
            TypeSerializeHint::Range { min, max } => {
                if let Some(max) = max {
                    TypeSerializeHint::Range { min: min * n, max: Some(max * n) }
                } else {
                    TypeSerializeHint::Range { min: min * n, max: None }
                }
            }
            TypeSerializeHint::None => TypeSerializeHint::None,
        }
    }

    /// Combines two [`TypeSerializeHint`]s into one,
    /// returning the overall hint.
    ///
    /// Follows the logic:
    ///   - `Exact` + `Exact` = `Exact`
    ///   - `Exact` + `Range` = `Range`
    ///   - `Range` + `Range` = `Range`
    ///   - `None` + `_` = `None`
    #[must_use]
    const fn add(self, other: Self) -> Self {
        match (self, other) {
            // Exact + Exact = Exact
            (TypeSerializeHint::Exact { size: a }, TypeSerializeHint::Exact { size: b }) => {
                TypeSerializeHint::Exact { size: a + b }
            }
            // Exact + Range = Range
            (TypeSerializeHint::Exact { size: exact }, TypeSerializeHint::Range { min, max })
            | (TypeSerializeHint::Range { min, max }, TypeSerializeHint::Exact { size: exact }) => {
                if let Some(max) = max {
                    TypeSerializeHint::Range { min: min + exact, max: Some(max + exact) }
                } else {
                    TypeSerializeHint::Range { min: min + exact, max: None }
                }
            }
            // Range + Range = Range
            (
                TypeSerializeHint::Range { min: min_a, max: max_a },
                TypeSerializeHint::Range { min: min_b, max: max_b },
            ) => TypeSerializeHint::Range {
                min: min_a + min_b,
                max: match (max_a, max_b) {
                    (Some(ma), Some(mb)) => Some(ma + mb),
                    _ => None,
                },
            },
            // None + Anything = None
            (TypeSerializeHint::None, _) | (_, TypeSerializeHint::None) => TypeSerializeHint::None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

const VAR_U16_HINT: TypeSerializeHint = TypeSerializeHint::Range { min: 1, max: Some(3) };
const VAR_U32_HINT: TypeSerializeHint = TypeSerializeHint::Range { min: 1, max: Some(5) };
const VAR_U32_UNBOUNDED_HINT: TypeSerializeHint = TypeSerializeHint::Range { min: 1, max: None };
const VAR_U64_HINT: TypeSerializeHint = TypeSerializeHint::Range { min: 1, max: Some(10) };
const VAR_U128_HINT: TypeSerializeHint = TypeSerializeHint::Range { min: 1, max: Some(19) };

/// A helper function to calculate the [`TypeSerializeHint`] for a [`Shape`].
pub(crate) const fn calculate_shape_hint(
    shape: &'static Shape,
    attrs: Option<&'static [FieldAttribute]>,
) -> TypeSerializeHint {
    match shape.def {
        // If key and value are zero-sized use repr hint,
        // otherwise use min length repr + unknown max
        Def::Map(MapDef { k, v, .. }) => {
            if let TypeSerializeHint::Exact { size: 0 } = calculate_shape_hint(k, attrs)
                && let TypeSerializeHint::Exact { size: 0 } = calculate_shape_hint(v, attrs)
            {
                VAR_U32_HINT
            } else {
                VAR_U32_UNBOUNDED_HINT
            }
        }

        // If inner/key is zero-sized use repr hint,
        // otherwise use min length repr + unknown max
        Def::Set(SetDef { t, .. })
        | Def::List(ListDef { t, .. })
        | Def::Slice(SliceDef { t, .. }) => {
            if let TypeSerializeHint::Exact { size: 0 } = calculate_shape_hint(t, attrs) {
                VAR_U32_HINT
            } else {
                VAR_U32_UNBOUNDED_HINT
            }
        }

        // Inner hint * length
        Def::Array(def) => calculate_shape_hint(def.t, attrs).multiply(def.n),

        // Boolean repr + inner hint
        Def::Option(def) => {
            let hint = calculate_shape_hint(def.t, attrs);
            if let Some(max) = hint.maximum() {
                TypeSerializeHint::Range { min: 1, max: Some(1 + max) }
            } else {
                VAR_U32_UNBOUNDED_HINT
            }
        }

        // Boolean repr + range of min/max
        Def::Result(def) => {
            let mut min = Some(usize::MAX);
            let mut max = Some(usize::MIN);

            let mut index = 0;
            while index < 2 {
                let hint = [def.t, def.e][index];
                index += 1;

                let hint = calculate_shape_hint(hint, attrs);

                // Update the minimum size
                if let Some(min) = min.as_mut()
                    && let Some(hint) = hint.minimum()
                {
                    if *min > hint {
                        *min = hint;
                    }
                } else {
                    min = None;
                }

                // Update the maximum size
                if let Some(max) = max.as_mut()
                    && let Some(hint) = hint.maximum()
                {
                    if *max < hint {
                        *max = hint;
                    }
                } else {
                    max = None;
                }
            }

            TypeSerializeHint::Exact { size: 1 }.add(TypeSerializeHint::from_min_max(min, max))
        }

        // Use the pointee `Shape`,
        // otherwise fallback to `Type` hint calculation
        Def::Pointer(def) => {
            if let Some(shape) = def.pointee {
                calculate_shape_hint(shape, attrs)
            } else {
                calculate_ty_hint(shape, attrs)
            }
        }

        // Fallback to `Type` hint calculation
        Def::Scalar | Def::Undefined => calculate_ty_hint(shape, attrs),

        _ => TypeSerializeHint::None,
    }
}

/// A helper function to calculate the [`TypeSerializeHint`] using the [`Type`]
/// of a [`Shape`].
const fn calculate_ty_hint(
    shape: &'static Shape,
    attrs: Option<&'static [FieldAttribute]>,
) -> TypeSerializeHint {
    match shape.ty {
        Type::Primitive(ty) => match ty {
            // `bool`
            PrimitiveType::Boolean => TypeSerializeHint::Exact { size: 1 },
            // `u8`, `u16`, `u32`, `u64`, `u128`, `f32`, `f64`
            PrimitiveType::Numeric(ty) => {
                let ShapeLayout::Sized(layout) = shape.layout else {
                    // `Unsized` types have no size hint
                    return TypeSerializeHint::None;
                };

                // Check the field attributes for serialization hints
                let mut variable_length = false;
                if let Some(field_attrs) = attrs {
                    let mut index: usize = 0;
                    while index < field_attrs.len() {
                        let attr = &field_attrs[index];
                        index += 1;

                        if let Some(name) = &attr.ns
                            && matches!(name.as_bytes(), b"mc")
                        {
                            match attr.key.as_bytes() {
                                // Mark as variable-length
                                b"variable" => {
                                    variable_length = true;
                                }
                                // Custom functions cannot provide a size hint
                                b"serialize" | b"deserialize" => {
                                    return TypeSerializeHint::None;
                                }
                                _ => {}
                            }
                        }
                    }
                }

                match (ty, variable_length) {
                    // Standard u8/u16/u32/u64/u128
                    (NumericType::Integer { .. }, false) => {
                        TypeSerializeHint::Exact { size: layout.size() }
                    }
                    // Standard f32/f64
                    (NumericType::Float, false) => TypeSerializeHint::Exact { size: layout.size() },
                    // Variable-length u16/u32/u64/u128
                    (NumericType::Integer { .. }, true) => match layout.size() {
                        2 => VAR_U16_HINT,
                        4 => VAR_U32_HINT,
                        8 => VAR_U64_HINT,
                        16 => VAR_U128_HINT,
                        _ => TypeSerializeHint::None,
                    },
                    // Variable-length f32/f64 (not supported)
                    (NumericType::Float, true) => TypeSerializeHint::None,
                }
            }
            PrimitiveType::Textual(ty) => match ty {
                // `str`
                TextualType::Str => VAR_U32_UNBOUNDED_HINT,
                // `char` (not supported)
                TextualType::Char => TypeSerializeHint::None,
            },
            // `!` (not supported)
            PrimitiveType::Never => TypeSerializeHint::None,
        },

        Type::Sequence(ty) => match ty {
            // `[$ty; N]`: Inner hint * length
            SequenceType::Array(ty) => calculate_shape_hint(ty.t, None).multiply(ty.n),
            // `[$ty]`: VarInt length repr + unknown max
            SequenceType::Slice(_) => VAR_U32_UNBOUNDED_HINT,
        },

        Type::User(ty) => match ty {
            // `struct`: Sum of field hints
            UserType::Struct(ty) => calculate_field_hint(ty.fields),
            // `enum`: VarInt + range of min/max variant hints
            UserType::Enum(ty) => {
                // Note: Represented as a variable-length integer
                let repr = VAR_U32_HINT;

                // Find the min/max size of all variants
                let mut index = 0;
                let mut min = Some(usize::MAX);
                let mut max = Some(usize::MIN);

                while index < ty.variants.len() {
                    // Get the hint for the variant
                    let variant = &ty.variants[index];
                    let hint = calculate_field_hint(variant.data.fields);
                    index += 1;

                    // Update the minimum size
                    if let Some(min) = min.as_mut()
                        && let Some(hint) = hint.minimum()
                    {
                        if *min > hint {
                            *min = hint;
                        }
                    } else {
                        min = None;
                    }

                    // Update the maximum size
                    if let Some(max) = max.as_mut()
                        && let Some(hint) = hint.maximum()
                    {
                        if *max < hint {
                            *max = hint;
                        }
                    } else {
                        max = None;
                    }
                }

                // repr + range min/max
                repr.add(TypeSerializeHint::from_min_max(min, max))
            }
            // `opaque`
            UserType::Opaque => {
                // Essentially overrides for specific known types
                // TODO: Use `ConstTypeId`/`TypeId` instead of identifiers
                match shape.type_identifier.as_bytes() {
                    // VarInt length repr + unknown max
                    b"String" => VAR_U32_UNBOUNDED_HINT,
                    // `[u8; 16]`
                    b"Uuid" => TypeSerializeHint::Exact { size: 16 },
                    _ => TypeSerializeHint::None,
                }
            }
            // `union` (not supported)
            UserType::Union(_) => TypeSerializeHint::None,
        },

        Type::Pointer(ty) => match ty {
            // `&T` or `&mut T`
            PointerType::Reference(ty) => calculate_shape_hint(ty.target, attrs),
            // `*const T` or `*mut T` (not supported)
            PointerType::Raw(_ty) => TypeSerializeHint::None,
            // `fn(..)` (not supported)
            PointerType::Function(_) => TypeSerializeHint::None,
        },

        // Undefined types (not supported)
        Type::Undefined => TypeSerializeHint::None,
    }
}

/// A helper function to calculate the [`TypeSerializeHint`] for a list of
/// [`Field`]s.
///
/// TODO: Access `Shape`s in a `const fn` :(
const fn calculate_field_hint(fields: &[Field]) -> TypeSerializeHint {
    let mut acc = TypeSerializeHint::Exact { size: 0 };

    let mut index = 0;
    while index < fields.len() {
        // let field = &fields[index];
        index += 1;

        // acc = acc.with(calculate_shape_hint(field.shape()));
        acc = TypeSerializeHint::None;
    }

    acc
}
