//! An iterator over the [`Field`]s of a [`Shape`].
#![allow(dead_code, reason = "WIP")]

use alloc::{vec, vec::Vec};
use core::ops::Range;

use facet::{Field, Shape, Type, UserType, Variant};

/// A recursive iterator over the fields of a [`Shape`].
pub(crate) struct ShapeFieldIter<'de> {
    steps: Vec<FieldIterType<'de>>,
}

enum FieldIterType<'de> {
    Struct(&'de [Field], Range<usize>),
    Enum(&'de [Variant]),
    Field(&'de Field),
    Shape(&'de Shape),
}

/// Either a [`Field`] or a [`Shape`].
#[derive(Debug, Clone, Copy)]
pub(crate) enum FieldOrShape<'de> {
    Field(&'de Field),
    Shape(&'de Shape),
}

impl<'de> FieldOrShape<'de> {
    /// Get the [`Shape`] represented by this [`FieldOrShape`].
    #[must_use]
    pub(crate) fn shape(&self) -> &'de Shape {
        match self {
            FieldOrShape::Field(field) => field.shape(),
            FieldOrShape::Shape(shape) => shape,
        }
    }
}

impl<'de> From<&'de Shape> for FieldOrShape<'de> {
    fn from(shape: &'de Shape) -> Self { FieldOrShape::Shape(shape) }
}
impl<'de> From<&'de Field> for FieldOrShape<'de> {
    fn from(field: &'de Field) -> Self { FieldOrShape::Field(field) }
}

// -------------------------------------------------------------------------------------------------

impl<'de> ShapeFieldIter<'de> {
    /// Create a new [`ShapeFieldIter`] for the given [`Shape`].
    pub(crate) fn new(shape: &'de Shape) -> Self { Self { steps: vec![FieldIterType::new(shape)] } }

    /// Returns `true` if the iterator is empty.
    #[must_use]
    pub(crate) const fn is_empty(&self) -> bool { self.steps.is_empty() }

    /// Get the next [`Shape`] in the iteration,
    /// optionally specifying an enum's variant.
    pub(crate) fn next_field(&mut self, variant: Option<usize>) -> Option<FieldOrShape<'de>> {
        match self.steps.last_mut()? {
            FieldIterType::Struct(fields, range) => {
                if let Some(next) = range.next().map(|i| &fields[i]) {
                    // Push the next field's shape onto the stack.
                    self.steps.push(FieldIterType::new(next.shape()));
                } else {
                    // Remove this struct from the stack.
                    self.steps.pop()?;
                }
                // Continue to the next field.
                self.next_field(variant)
            }
            FieldIterType::Enum(variants) => {
                // Get the selected variant.
                let selected = &variants[variant.expect("Expected an enum variant index!")];
                // Remove this enum from the stack.
                self.steps.pop()?;
                // Push the selected variant's fields onto the stack.
                self.steps.push(FieldIterType::Struct(
                    selected.data.fields,
                    0..selected.data.fields.len(),
                ));
                // Continue to the next field.
                self.next_field(None)
            }
            // Return the field
            FieldIterType::Field(_) => self.steps.pop().map(|step| match step {
                FieldIterType::Field(field) => FieldOrShape::Field(field),
                _ => unreachable!(),
            }),
            // Return the shape
            FieldIterType::Shape(_) => self.steps.pop().map(|step| match step {
                FieldIterType::Shape(shape) => FieldOrShape::Shape(shape),
                _ => unreachable!(),
            }),
        }
    }
}

impl<'de> FieldIterType<'de> {
    /// Create a new [`FieldIterType`] for the given [`Shape`].
    const fn new(shape: &'de Shape) -> Self {
        match shape.ty {
            Type::User(UserType::Struct(ty)) => Self::Struct(ty.fields, 0..ty.fields.len()),
            Type::User(UserType::Enum(ty)) => FieldIterType::Enum(ty.variants),
            _ => FieldIterType::Shape(shape),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
fn assert_field(iter: Option<FieldOrShape<'_>>, field: Option<&Shape>) {
    assert_eq!(iter.map(|f| f.shape()), field)
}

#[test]
fn iterate_primitive() {
    use facet::Facet;

    let mut iter = ShapeFieldIter::new(bool::SHAPE);
    assert_field(iter.next_field(None), Some(bool::SHAPE));
    assert_field(iter.next_field(None), None);

    let mut iter = ShapeFieldIter::new(u8::SHAPE);
    assert_field(iter.next_field(None), Some(u8::SHAPE));
    assert_field(iter.next_field(None), None);

    let mut iter = ShapeFieldIter::new(u32::SHAPE);
    assert_field(iter.next_field(None), Some(u32::SHAPE));
    assert_field(iter.next_field(None), None);

    let mut iter = ShapeFieldIter::new(f32::SHAPE);
    assert_field(iter.next_field(None), Some(f32::SHAPE));
    assert_field(iter.next_field(None), None);
}

#[test]
fn iterate_newtype() {
    use facet::Facet;

    #[derive(Facet)]
    struct Newtype(u32);

    #[derive(Facet)]
    struct NestedNewtype(Newtype);

    let mut iter = ShapeFieldIter::new(Newtype::SHAPE);
    assert_field(iter.next_field(None), Some(u32::SHAPE));
    assert_field(iter.next_field(None), None);

    let mut iter = ShapeFieldIter::new(NestedNewtype::SHAPE);
    assert_field(iter.next_field(None), Some(u32::SHAPE));
    assert_field(iter.next_field(None), None);
}

#[test]
fn iterate_struct() {
    use facet::Facet;

    #[derive(Facet)]
    struct TestStruct {
        a: u8,
        b: bool,
        c: u32,
    }

    #[derive(Facet)]
    struct NestedStruct {
        x: TestStruct,
        y: TestStruct,
        z: Vec<u64>,
    }

    let mut iter = ShapeFieldIter::new(TestStruct::SHAPE);
    assert_field(iter.next_field(None), Some(u8::SHAPE));
    assert_field(iter.next_field(None), Some(bool::SHAPE));
    assert_field(iter.next_field(None), Some(u32::SHAPE));
    assert_field(iter.next_field(None), None);

    let mut iter = ShapeFieldIter::new(NestedStruct::SHAPE);
    assert_field(iter.next_field(None), Some(u8::SHAPE));
    assert_field(iter.next_field(None), Some(bool::SHAPE));
    assert_field(iter.next_field(None), Some(u32::SHAPE));
    assert_field(iter.next_field(None), Some(u8::SHAPE));
    assert_field(iter.next_field(None), Some(bool::SHAPE));
    assert_field(iter.next_field(None), Some(u32::SHAPE));
    assert_field(iter.next_field(None), Some(Vec::<u64>::SHAPE));
    assert_field(iter.next_field(None), None);
}

#[test]
fn iterate_enum() {
    #![expect(dead_code, reason = "Tests")]

    use facet::Facet;

    #[repr(u8)]
    #[derive(Facet)]
    enum TestEnum {
        A(u8, bool),
        B { x: u32, y: u64 },
        C,
    }

    #[derive(Facet)]
    struct EnumWrapper {
        a: TestEnum,
        b: u16,
        c: TestEnum,
    }

    // Variant A
    let mut iter = ShapeFieldIter::new(TestEnum::SHAPE);
    assert_field(iter.next_field(Some(0)), Some(u8::SHAPE));
    assert_field(iter.next_field(None), Some(bool::SHAPE));
    assert_field(iter.next_field(None), None);

    // Variant B
    let mut iter = ShapeFieldIter::new(TestEnum::SHAPE);
    assert_field(iter.next_field(Some(1)), Some(u32::SHAPE));
    assert_field(iter.next_field(None), Some(u64::SHAPE));
    assert_field(iter.next_field(None), None);

    // Variant C
    let mut iter = ShapeFieldIter::new(TestEnum::SHAPE);
    assert_field(iter.next_field(Some(2)), None);
    assert_field(iter.next_field(None), None);

    // EnumWrapper
    let mut iter = ShapeFieldIter::new(EnumWrapper::SHAPE);
    assert_field(iter.next_field(Some(0)), Some(u8::SHAPE));
    assert_field(iter.next_field(None), Some(bool::SHAPE));
    assert_field(iter.next_field(None), Some(u16::SHAPE));
    assert_field(iter.next_field(Some(1)), Some(u32::SHAPE));
    assert_field(iter.next_field(None), Some(u64::SHAPE));
    assert_field(iter.next_field(None), None);
}
