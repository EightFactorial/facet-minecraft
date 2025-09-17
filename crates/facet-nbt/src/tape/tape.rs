use alloc::vec::Vec;
use core::{fmt::Debug, num::NonZeroUsize};

use super::{NbtTapeError, NbtTapeItem, NbtTapeTag};
use crate::mutf8::Mutf8Str;

/// A tape-based representation of a slice of NBT data.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NbtTape<'a> {
    input: &'a [u8],
    name: Option<&'a Mutf8Str>,
    storage: Vec<NbtTapeItem>,
}

impl<'a> NbtTape<'a> {
    /// Get the slice of data used to create this tape.
    #[must_use]
    pub const fn input(&self) -> &[u8] { self.input }

    /// Get the name of the root tag, if it has one.
    #[must_use]
    pub const fn name(&self) -> Option<&'a Mutf8Str> { self.name }

    /// Get the storage slice containing all [`NbtTapeItem`]s in this tape.
    #[must_use]
    pub const fn storage(&self) -> &[NbtTapeItem] { self.storage.as_slice() }

    /// Trim the input slice to only include the bytes used by this tape.
    ///
    /// Returns any remaining bytes after the tape.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "Position will never exceed `usize`")]
    pub const fn trim(&mut self) -> &'a [u8] {
        if let Some(last) = self.storage.as_slice().last()
            && let Some((data, remaining)) = self.input.split_at_checked(last.position() as usize)
        {
            self.input = data;
            remaining
        } else {
            &[]
        }
    }
}

impl Debug for NbtTape<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(
            &NbtTapeSlice {
                input: self.input,
                name: self.name.unwrap_or(Mutf8Str::EMPTY),
                storage: &self.storage,
            },
            f,
        )
    }
}

// -------------------------------------------------------------------------------------------------

/// A slice of an [`NbtTape`].
///
/// Used when accessing a portion of the tape.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NbtTapeSlice<'a, 'b> {
    input: &'a [u8],
    name: &'a Mutf8Str,
    storage: &'b [NbtTapeItem],
}

impl<'a, 'b> NbtTapeSlice<'a, 'b> {
    /// Get the slice of data used to create this tape.
    #[must_use]
    pub const fn input(&self) -> &'a [u8] { self.input }

    /// Get the name of this slice.
    #[must_use]
    pub const fn name(&self) -> &'a Mutf8Str { self.name }

    /// Get the storage containing all [`NbtTapeItem`]s in this slice.
    #[must_use]
    pub const fn storage(&self) -> &'b [NbtTapeItem] { self.storage }
}

impl Debug for NbtTapeSlice<'_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut f = f.debug_struct("NbtTapeSlice");
        f.field("name", &self.name);
        f.field("input", &format_args!("&[u8; {}]", self.input.len()));
        f.field("storage", &self.storage);
        f.finish_non_exhaustive()
    }
}

// -------------------------------------------------------------------------------------------------

impl<'a> NbtTape<'a> {
    /// Read a named [`NbtTape`] from a slice of bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is not valid NBT data.
    pub fn named_from_slice(input: &'a [u8]) -> Result<Self, NbtTapeError> {
        match input.first().copied() {
            Some(NbtTapeTag::END) => Ok(Self { input, name: None, storage: Vec::new() }),
            Some(NbtTapeTag::COMPOUND) => {
                let (_name_length, name_read) = read_string(input, 1)?;
                let name = input.get(1 + name_read..1 + name_read).ok_or(NbtTapeError)?;
                // TODO: Validate MUTF-8
                let name = unsafe { Mutf8Str::from_bytes_unchecked(name) };

                let start = NonZeroUsize::new(1 + name_read).ok_or(NbtTapeError)?;
                let storage = Self::from_slice_inner(start, input)?;
                Ok(Self { input, name: Some(name), storage })
            }
            _ => Err(NbtTapeError),
        }
    }

    /// Read a unnamed [`NbtTape`] from a slice of bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is not valid NBT data.
    pub fn unnamed_from_slice(input: &'a [u8]) -> Result<Self, NbtTapeError> {
        match input.first().copied() {
            Some(NbtTapeTag::END) => Ok(Self { input, name: None, storage: Vec::new() }),
            Some(NbtTapeTag::COMPOUND) => {
                let start = NonZeroUsize::new(1).ok_or(NbtTapeError)?;
                let storage = Self::from_slice_inner(start, input)?;
                Ok(Self { input, name: None, storage })
            }
            _ => Err(NbtTapeError),
        }
    }

    /// The internal tape parsing function.
    #[expect(clippy::too_many_lines, reason = "Inner `NbtTape` parser")]
    fn from_slice_inner(
        start: NonZeroUsize,
        input: &'a [u8],
    ) -> Result<Vec<NbtTapeItem>, NbtTapeError> {
        // A vector to hold the parsed items.
        let mut storage = Vec::with_capacity(input.len().saturating_div(8));

        // A cursor and depth counter to track our position.
        let mut cursor = Some(start);
        let mut depth = 0u32;

        #[cfg(feature = "trace")]
        {
            tracing::trace!("Starting Root!");
            tracing::trace!("Parsing {} bytes, starting at {start}", input.len());
        }

        // Take the cursor position and get that byte.
        while let Some(mut index) = cursor.take().map(NonZeroUsize::get) {
            let Some(&byte) = input.get(index) else { return Err(NbtTapeError) };

            #[cfg(feature = "trace")]
            {
                tracing::trace!("--------------------------------");
                tracing::trace!("Last: {:?}", storage.last());
                tracing::trace!("Depth: {depth}");
                tracing::trace!("Cursor: {index}, byte={byte}");
                tracing::trace!(
                    "Preview ({}..{}): {:?}",
                    index.saturating_sub(4),
                    index.saturating_add(16),
                    input
                        .get(index.saturating_sub(4)..index.saturating_add(16).min(input.len()))
                        .unwrap()
                );
            }

            // If we hit an `End` tag, decrease depth and continue.
            if matches!(byte, NbtTapeTag::END) {
                // Push the end tag to the storage.
                storage.push(NbtTapeItem::new(NbtTapeTag::End, index as u64 + 1, 0));

                #[cfg(feature = "trace")]
                tracing::trace!("Tag End");

                // If we finish the root compound, we're done!
                if depth == 0 {
                    #[cfg(feature = "trace")]
                    {
                        tracing::trace!("--------------------------------");
                        tracing::trace!("Finished Root!");
                    }

                    return Ok(storage);
                }

                // Otherwise, pop the state and continue.
                depth -= 1;
                cursor = NonZeroUsize::new(index + 1);

                continue;
            }

            // Read the name of the tag.
            {
                let (length, read) = read_string(input, index + 1)?;
                storage.push(NbtTapeItem::new(NbtTapeTag::String, index as u64 + 1, length));

                #[cfg(feature = "trace")]
                if let Some(mbytes) = input.get(index + 2..index + 2 + length as usize)
                    && let Ok(mstr) = Mutf8Str::try_from_utf8(mbytes)
                {
                    tracing::trace!("Tag Name: {mstr:?}");
                } else {
                    tracing::trace!("Tag Name: <invalid>");
                }

                // Move the index past the name.
                index += read;
            }

            // Handle the tag type.
            match byte {
                NbtTapeTag::BYTE => {
                    // Push a `Byte` tag and move forward.
                    cursor = NonZeroUsize::new(index + 1 + 1);
                    storage.push(NbtTapeItem::new(NbtTapeTag::Byte, index as u64 + 1, 0));

                    #[cfg(feature = "trace")]
                    if let Some(&value) = input.get(index + 1) {
                        tracing::trace!("Byte Value: {value}");
                    } else {
                        tracing::trace!("Byte Value: <invalid>");
                    }
                }
                NbtTapeTag::SHORT => {
                    // Push a `Short` tag and move forward.
                    cursor = NonZeroUsize::new(index + 1 + 2);
                    storage.push(NbtTapeItem::new(NbtTapeTag::Short, index as u64 + 1, 0));

                    #[cfg(feature = "trace")]
                    if let Some(slice) = input.get(index + 1..index + 3) {
                        // SAFETY: The slice is guaranteed to be 2 bytes long, as specified above.
                        let value =
                            u16::from_be_bytes(unsafe { slice.try_into().unwrap_unchecked() });
                        tracing::trace!("Short Value: {value}");
                    } else {
                        tracing::trace!("Short Value: <invalid>");
                    }
                }
                NbtTapeTag::INT => {
                    // Push an `Int` tag and move forward.
                    cursor = NonZeroUsize::new(index + 1 + 4);
                    storage.push(NbtTapeItem::new(NbtTapeTag::Int, index as u64 + 1, 0));

                    #[cfg(feature = "trace")]
                    if let Some(slice) = input.get(index + 1..index + 5) {
                        // SAFETY: The slice is guaranteed to be 4 bytes long, as specified above.
                        let value =
                            u32::from_be_bytes(unsafe { slice.try_into().unwrap_unchecked() });
                        tracing::trace!("Int Value: {value}");
                    } else {
                        tracing::trace!("Int Value: <invalid>");
                    }
                }
                NbtTapeTag::LONG => {
                    // Push the `Long` tag and move forward.
                    cursor = NonZeroUsize::new(index + 1 + 8);
                    storage.push(NbtTapeItem::new(NbtTapeTag::Long, index as u64 + 1, 0));

                    #[cfg(feature = "trace")]
                    if let Some(slice) = input.get(index..index + 8) {
                        // SAFETY: The slice is guaranteed to be 8 bytes long, as specified above.
                        let value =
                            u64::from_be_bytes(unsafe { slice.try_into().unwrap_unchecked() });
                        tracing::trace!("Long Value: {value}");
                    } else {
                        tracing::trace!("Long Value: <invalid>");
                    }
                }
                NbtTapeTag::FLOAT => {
                    // Push a `Float` tag and move forward.
                    cursor = NonZeroUsize::new(index + 1 + 4);
                    storage.push(NbtTapeItem::new(NbtTapeTag::Float, index as u64 + 1, 0));

                    #[cfg(feature = "trace")]
                    if let Some(slice) = input.get(index..index + 4) {
                        // SAFETY: The slice is guaranteed to be 4 bytes long, as specified above.
                        let value =
                            f32::from_be_bytes(unsafe { slice.try_into().unwrap_unchecked() });
                        tracing::trace!("Float Value: {value}");
                    } else {
                        tracing::trace!("Float Value: <invalid>");
                    }
                }
                NbtTapeTag::DOUBLE => {
                    // Push a `Double` tag and move forward.
                    cursor = NonZeroUsize::new(index + 1 + 8);
                    storage.push(NbtTapeItem::new(NbtTapeTag::Double, index as u64 + 1, 0));

                    #[cfg(feature = "trace")]
                    if let Some(slice) = input.get(index..index + 8) {
                        // SAFETY: The slice is guaranteed to be 8 bytes long, as specified above.
                        let value =
                            f64::from_be_bytes(unsafe { slice.try_into().unwrap_unchecked() });
                        tracing::trace!("Double Value: {value}");
                    } else {
                        tracing::trace!("Double Value: <invalid>");
                    }
                }
                NbtTapeTag::BYTE_ARRAY => {
                    // Read the `ByteArray` and move forward.
                    let (length, read) = read_array(input, index + 1, 1)?;
                    cursor = NonZeroUsize::new(index + 1 + read);
                    storage.push(NbtTapeItem::new(NbtTapeTag::ByteArray, index as u64 + 1, length));

                    #[cfg(feature = "trace")]
                    tracing::trace!("Byte Array: [i8; {length}]");
                }
                NbtTapeTag::INT_ARRAY => {
                    // Read the `IntArray` and move forward.
                    let (length, read) = read_array(input, index + 1, 4)?;
                    cursor = NonZeroUsize::new(index + 1 + read);
                    storage.push(NbtTapeItem::new(NbtTapeTag::IntArray, index as u64 + 1, length));

                    #[cfg(feature = "trace")]
                    tracing::trace!("Int Array: [i32; {length}]");
                }
                NbtTapeTag::LONG_ARRAY => {
                    // Read the `LongArray` and move forward.
                    let (length, read) = read_array(input, index + 1, 8)?;
                    cursor = NonZeroUsize::new(index + 1 + read);
                    storage.push(NbtTapeItem::new(NbtTapeTag::LongArray, index as u64 + 1, length));

                    #[cfg(feature = "trace")]
                    tracing::trace!("Long Array: [i64; {length}]");
                }
                NbtTapeTag::STRING => {
                    // Read the `String` and move forward.
                    let (length, read) = read_string(input, index + 1)?;
                    cursor = NonZeroUsize::new(index + 1 + read);
                    storage.push(NbtTapeItem::new(NbtTapeTag::String, index as u64 + 1, length));

                    #[cfg(feature = "trace")]
                    if let Some(mbytes) = input.get(index + 3..index + 3 + length as usize)
                        && let Ok(mstr) = Mutf8Str::try_from_utf8(mbytes)
                    {
                        tracing::trace!("String Value: {mstr:?}");
                    } else {
                        tracing::trace!("String Value: <invalid>");
                    }
                }

                NbtTapeTag::COMPOUND => {
                    // Move forward and increase the depth.
                    cursor = NonZeroUsize::new(index + 1);
                    depth += 1;
                }

                NbtTapeTag::LIST => {
                    // Read the list type and length.
                    let &list_ty = input.get(index + 1).ok_or(NbtTapeError)?;
                    let (list_len, len_read) = read_length(input, index + 1 + 1)?;
                    index += 1 + len_read;

                    // Handle the list type.
                    match list_ty {
                        NbtTapeTag::END => {
                            // Add a `ListEmpty` tag and move forward.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListEmpty,
                                index as u64 + 1,
                                0,
                            ));
                            cursor = NonZeroUsize::new(index + 1);
                        }
                        NbtTapeTag::BYTE => {
                            // Add a `ListByte` tag and move forward.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListByte,
                                index as u64 + 1,
                                list_len,
                            ));
                            cursor = NonZeroUsize::new(index + 1 + list_len as usize);
                        }
                        NbtTapeTag::SHORT => {
                            // Add a `ListShort` tag and move forward.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListShort,
                                index as u64 + 1,
                                list_len,
                            ));
                            cursor = NonZeroUsize::new(index + 1 + (list_len as usize * 2));
                        }
                        NbtTapeTag::INT => {
                            // Add a `ListInt` tag and move forward.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListInt,
                                index as u64 + 1,
                                list_len,
                            ));
                            cursor = NonZeroUsize::new(index + 1 + (list_len as usize * 4));
                        }
                        NbtTapeTag::LONG => {
                            // Add a `ListLong` tag and move forward.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListLong,
                                index as u64 + 1,
                                list_len,
                            ));
                            cursor = NonZeroUsize::new(index + 1 + (list_len as usize * 8));
                        }
                        NbtTapeTag::FLOAT => {
                            // Add a `ListFloat` tag and move forward.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListFloat,
                                index as u64 + 1,
                                list_len,
                            ));
                            cursor = NonZeroUsize::new(index + 1 + (list_len as usize * 4));
                        }
                        NbtTapeTag::DOUBLE => {
                            // Add a `ListDouble` tag and move forward.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListDouble,
                                index as u64 + 1,
                                list_len,
                            ));
                            cursor = NonZeroUsize::new(index + 1 + (list_len as usize * 8));
                        }

                        NbtTapeTag::STRING => {
                            // Add a `ListString` tag.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListString,
                                index as u64 + 1,
                                list_len,
                            ));

                            // Read each string and push a `String` tag.
                            let mut acc = 0usize;
                            for _ in 0..list_len {
                                let (str_len, str_read) = read_string(input, index + 1 + acc)?;
                                acc += str_read;

                                storage.push(NbtTapeItem::new(
                                    NbtTapeTag::String,
                                    index as u64 + 1,
                                    str_len,
                                ));
                            }

                            // Move the cursor forward.
                            cursor = NonZeroUsize::new(index + 1 + acc);
                        }

                        NbtTapeTag::BYTE_ARRAY => {
                            // Add a `ListByteArray` tag.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListByteArray,
                                index as u64 + 1,
                                list_len,
                            ));

                            // Read each byte array and push a `ByteArray` tag.
                            let mut acc = 0usize;
                            for _ in 0..list_len {
                                let (arr_len, arr_read) = read_array(input, index + 1 + acc, 1)?;
                                acc += arr_read;

                                storage.push(NbtTapeItem::new(
                                    NbtTapeTag::ByteArray,
                                    index as u64 + 1,
                                    arr_len,
                                ));
                            }

                            // Move the cursor forward.
                            cursor = NonZeroUsize::new(index + 1 + acc);
                        }
                        NbtTapeTag::INT_ARRAY => {
                            // Add a `ListIntArray` tag.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListIntArray,
                                index as u64 + 1,
                                list_len,
                            ));

                            // Read each int array and push an `IntArray` tag.
                            let mut acc = 0usize;
                            for _ in 0..list_len {
                                let (arr_len, arr_read) = read_array(input, index + 1 + acc, 4)?;
                                acc += arr_read;

                                storage.push(NbtTapeItem::new(
                                    NbtTapeTag::IntArray,
                                    index as u64 + 1,
                                    arr_len,
                                ));
                            }

                            // Move the cursor forward.
                            cursor = NonZeroUsize::new(index + 1 + acc);
                        }
                        NbtTapeTag::LONG_ARRAY => {
                            // Add a `ListLongArray` tag.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListLongArray,
                                index as u64 + 1,
                                list_len,
                            ));

                            // Read each long array and push a `LongArray` tag.
                            let mut acc = 0usize;
                            for _ in 0..list_len {
                                let (arr_len, arr_read) = read_array(input, index + 1 + acc, 8)?;
                                acc += arr_read;

                                storage.push(NbtTapeItem::new(
                                    NbtTapeTag::LongArray,
                                    index as u64 + 1,
                                    arr_len,
                                ));
                            }

                            // Move the cursor forward.
                            cursor = NonZeroUsize::new(index + 1 + acc);
                        }

                        // TODO: Handle nested lists.
                        NbtTapeTag::LIST => todo!(),

                        NbtTapeTag::COMPOUND => {
                            // Add a `ListCompound` tag and increase depth.
                            storage.push(NbtTapeItem::new(
                                NbtTapeTag::ListCompound,
                                index as u64 + 1,
                                list_len,
                            ));
                            depth += list_len;

                            // Move the cursor forward.
                            cursor = NonZeroUsize::new(index + 1);
                        }

                        _ => return Err(NbtTapeError),
                    }
                }

                _ => return Err(NbtTapeError),
            }
        }

        Err(NbtTapeError)
    }
}

/// Read a big-endian length prefix,
/// returning the length and the number of bytes read.
fn read_length(input: &[u8], index: usize) -> Result<(u32, usize), NbtTapeError> {
    let slice = input.get(index..index + 4).ok_or(NbtTapeError)?;
    // SAFETY: The length is guaranteed to be 4 bytes long, as specified above.
    Ok((u32::from_be_bytes(unsafe { slice.try_into().unwrap_unchecked() }), 4))
}

// Read a length-prefixed array,
// returning the length of the array and the number of bytes read.
fn read_array(input: &[u8], index: usize, size: usize) -> Result<(u32, usize), NbtTapeError> {
    read_length(input, index)
        .map(|(length, read)| (length, read + (length as usize).saturating_mul(size)))
}

/// Read a length-prefixed string,
/// returning the length of the string and the number of bytes read.
fn read_string(input: &[u8], index: usize) -> Result<(u32, usize), NbtTapeError> {
    let slice = input.get(index..index + 2).ok_or(NbtTapeError)?;
    // SAFETY: The length is guaranteed to be 2 bytes long, as specified above.
    let length = u32::from(u16::from_be_bytes(unsafe { slice.try_into().unwrap_unchecked() }));

    Ok((length, length as usize + 2))
}
