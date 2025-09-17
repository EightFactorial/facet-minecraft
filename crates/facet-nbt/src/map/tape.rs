use alloc::{vec, vec::Vec};

use super::{ByteCow, NbtItem, NbtListItem, NbtMap};
use crate::{
    Mutf8Str,
    tape::{NbtTape, NbtTapeTag, NbtTapeValidationError},
};

impl<'a> NbtMap<'a> {
    /// Attempt to construct an [`NbtMap`] from an [`NbtTape`].
    ///
    /// # Errors
    ///
    /// Returns an error if the tape is an invalid NBT structure.
    #[expect(clippy::cast_possible_truncation, reason = "This won't ever happen")]
    #[expect(clippy::too_many_lines, reason = "Requires handling all NBT types")]
    pub fn try_from_tape(tape: &'a NbtTape<'a>) -> Result<Self, NbtTapeValidationError> {
        let mut stack = vec![NbtMap::new_none()];
        let mut stack_name = Vec::new();
        let mut name = Option::None;

        #[cfg(feature = "trace")]
        {
            tracing::trace!("Starting Root!");
            tracing::trace!("Parsing {} items", tape.storage().len());
        }

        for item in tape.storage() {
            #[cfg(feature = "trace")]
            {
                tracing::trace!("--------------------------------");
                tracing::trace!("Depth: {}", stack.len());
                tracing::trace!("Item: {item:?}");
            }

            let (tag, pos, val) = item.into_parts();

            if matches!(tag, NbtTapeTag::End) {
                #[cfg(feature = "trace")]
                tracing::trace!("Tag End");

                let finished = stack.pop().ok_or(NbtTapeValidationError)?;
                if let Some(parent) = stack.last_mut() {
                    let name = stack_name.pop().ok_or(NbtTapeValidationError)?;
                    parent.insert(name, NbtItem::Compound(finished));
                } else {
                    #[cfg(feature = "trace")]
                    {
                        tracing::trace!("--------------------------------");
                        tracing::trace!("Finished Root!");
                    }

                    return Ok(finished);
                }
            }

            if name.is_none() {
                if matches!(tag, NbtTapeTag::String) {
                    let (pos, len) = (pos as usize, val as usize);
                    let bytes =
                        tape.input().get(pos + 2..pos + 2 + len).ok_or(NbtTapeValidationError)?;
                    let mstr =
                        Mutf8Str::try_from_utf8(bytes).map_err(|_| NbtTapeValidationError)?;
                    name = Some(mstr);
                    continue;
                }

                return Err(NbtTapeValidationError);
            }

            let map = stack.last_mut().ok_or(NbtTapeValidationError)?;
            let name = name.take().ok_or(NbtTapeValidationError)?;

            match tag {
                NbtTapeTag::End => unreachable!(),
                NbtTapeTag::Byte => {
                    let pos = pos as usize;
                    let byte = tape.input().get(pos).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Byte`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::Byte(unsafe {
                            ByteCow::from_slice_unchecked(core::slice::from_ref(byte))
                        }),
                    );
                }
                NbtTapeTag::Short => {
                    let pos = pos as usize;
                    let bytes = tape.input().get(pos..pos + 2).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Short`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::Short(unsafe { ByteCow::from_slice_unchecked(bytes) }),
                    );
                }
                NbtTapeTag::Int => {
                    let pos = pos as usize;
                    let bytes = tape.input().get(pos..pos + 4).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Int`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(name, NbtItem::Int(unsafe { ByteCow::from_slice_unchecked(bytes) }));
                }
                NbtTapeTag::Long => {
                    let pos = pos as usize;
                    let bytes = tape.input().get(pos..pos + 8).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Long`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::Long(unsafe { ByteCow::from_slice_unchecked(bytes) }),
                    );
                }
                NbtTapeTag::Float => {
                    let pos = pos as usize;
                    let bytes = tape.input().get(pos..pos + 4).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Float`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::Float(unsafe { ByteCow::from_slice_unchecked(bytes) }),
                    );
                }
                NbtTapeTag::Double => {
                    let pos = pos as usize;
                    let bytes = tape.input().get(pos..pos + 8).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Double`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::Double(unsafe { ByteCow::from_slice_unchecked(bytes) }),
                    );
                }

                NbtTapeTag::ByteArray => {
                    let (pos, len) = (pos as usize, val as usize);
                    let bytes =
                        tape.input().get(pos + 2..pos + 2 + len).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Byte`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::ByteArray(unsafe { ByteCow::from_slice_unchecked(bytes) }),
                    );
                }
                NbtTapeTag::IntArray => {
                    let (pos, len) = (pos as usize, val as usize);
                    let bytes = tape
                        .input()
                        .get(pos + 2..pos + 2 + len * 4)
                        .ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Int`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::IntArray(unsafe { ByteCow::from_slice_unchecked(bytes) }),
                    );
                }
                NbtTapeTag::LongArray => {
                    let (pos, len) = (pos as usize, val as usize);
                    let bytes = tape
                        .input()
                        .get(pos + 2..pos + 2 + len * 8)
                        .ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Long`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::LongArray(unsafe { ByteCow::from_slice_unchecked(bytes) }),
                    );
                }
                NbtTapeTag::String => {
                    let (pos, len) = (pos as usize, val as usize);
                    let bytes =
                        tape.input().get(pos + 2..pos + 2 + len).ok_or(NbtTapeValidationError)?;
                    let mstr =
                        Mutf8Str::try_from_utf8(bytes).map_err(|_| NbtTapeValidationError)?;

                    map.insert(name, NbtItem::String(mstr));
                }

                NbtTapeTag::Compound => {
                    stack_name.push(name);
                    stack.push(NbtMap::new_none());
                }

                NbtTapeTag::ListEmpty => {
                    map.insert(name, NbtItem::List(NbtListItem::Empty));
                }
                NbtTapeTag::ListByte => {
                    let (pos, len) = (pos as usize, val as usize);
                    let bytes = tape.input().get(pos..pos + len).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Byte`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::List(NbtListItem::Byte(unsafe {
                            ByteCow::from_slice_unchecked(bytes)
                        })),
                    );
                }
                NbtTapeTag::ListShort => {
                    let (pos, len) = (pos as usize, val as usize);
                    let bytes =
                        tape.input().get(pos..pos + (len * 2)).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Short`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::List(NbtListItem::Short(unsafe {
                            ByteCow::from_slice_unchecked(bytes)
                        })),
                    );
                }
                NbtTapeTag::ListInt => {
                    let (pos, len) = (pos as usize, val as usize);
                    let bytes =
                        tape.input().get(pos..pos + (len * 4)).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Int`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::List(NbtListItem::Int(unsafe {
                            ByteCow::from_slice_unchecked(bytes)
                        })),
                    );
                }
                NbtTapeTag::ListLong => {
                    let (pos, len) = (pos as usize, val as usize);
                    let bytes =
                        tape.input().get(pos..pos + (len * 8)).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Long`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::List(NbtListItem::Long(unsafe {
                            ByteCow::from_slice_unchecked(bytes)
                        })),
                    );
                }
                NbtTapeTag::ListFloat => {
                    let (pos, len) = (pos as usize, val as usize);
                    let bytes =
                        tape.input().get(pos..pos + (len * 4)).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Float`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::List(NbtListItem::Float(unsafe {
                            ByteCow::from_slice_unchecked(bytes)
                        })),
                    );
                }
                NbtTapeTag::ListDouble => {
                    let (pos, len) = (pos as usize, val as usize);
                    let bytes =
                        tape.input().get(pos..pos + (len * 8)).ok_or(NbtTapeValidationError)?;

                    // SAFETY: `Double`s are PODs, and the slice is guaranteed to be a valid length.
                    map.insert(
                        name,
                        NbtItem::List(NbtListItem::Double(unsafe {
                            ByteCow::from_slice_unchecked(bytes)
                        })),
                    );
                }

                NbtTapeTag::ListByteArray => todo!(),
                NbtTapeTag::ListIntArray => todo!(),
                NbtTapeTag::ListLongArray => todo!(),

                NbtTapeTag::ListString => todo!(),

                NbtTapeTag::ListCompound => todo!(),
            }
        }

        Err(NbtTapeValidationError)
    }
}
