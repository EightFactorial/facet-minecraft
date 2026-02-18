use alloc::vec::Vec;

use facet_format::{EnumVariantHint, ScalarTypeHint};

#[repr(transparent)]
#[derive(Debug)]
pub(super) struct DeserializerStack {
    stack: Vec<StackEntry>,
}

#[derive(Debug)]
pub(super) enum StackEntry {
    Struct { remaining: usize },
    Enum { variants: Vec<EnumVariantHint>, variant: Option<usize>, remaining: Option<usize> },

    Sequence { remaining: Option<usize> },
    Map { remaining: Option<usize> },
    Optional { present: Option<bool> },

    Scalar { hint: ScalarTypeHint },
}

// -------------------------------------------------------------------------------------------------

impl DeserializerStack {
    /// Create a new [`DeserializerStack`].
    #[inline]
    #[must_use]
    pub(super) const fn new() -> Self { Self { stack: Vec::new() } }

    /// Get a mutable reference to the next entry on the stack.
    #[inline]
    #[must_use]
    pub(super) const fn next_mut(&mut self) -> Option<&mut StackEntry> {
        self.stack.as_mut_slice().last_mut()
    }

    /// Pop the next entry off the stack.
    #[inline]
    #[must_use]
    pub(super) fn pop(&mut self) -> Option<StackEntry> { self.stack.pop() }

    pub(super) fn push_struct_hint(&mut self, fields: usize) {
        self.stack.push(StackEntry::Struct { remaining: fields });
    }

    pub(super) fn push_enum_hint(&mut self, variants: &[EnumVariantHint]) {
        self.stack.push(StackEntry::Enum {
            variants: variants.to_vec(),
            variant: None,
            remaining: None,
        });
    }

    pub(super) fn push_sequence_hint(&mut self, elements: Option<usize>) {
        self.stack.push(StackEntry::Sequence { remaining: elements });
    }

    pub(super) fn push_map_hint(&mut self) { self.stack.push(StackEntry::Map { remaining: None }); }

    pub(super) fn push_scalar_hint(&mut self, hint: ScalarTypeHint) {
        self.stack.push(StackEntry::Scalar { hint });
    }

    pub(super) fn push_optional_hint(&mut self) {
        self.stack.push(StackEntry::Optional { present: None });
    }
}
