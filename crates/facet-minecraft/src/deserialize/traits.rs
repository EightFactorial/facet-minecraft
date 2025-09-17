#[cfg(feature = "rich-diagnostics")]
use alloc::format;

use super::{DeserError, DeserErrorKind};

/// A trait for deserializing primitive protocol types.
pub trait Deserializer {
    /// Deserialize a unit from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid unit.
    #[inline]
    fn deserialize_unit<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<&'input [u8], DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `()`");

        Ok(input)
    }

    /// Deserialize a [`bool`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`bool`].
    fn deserialize_bool<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(bool, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `bool`");

        let (byte, rest) = self.deserialize_u8(input)?;
        match byte {
            0 => Ok((false, rest)),
            1 => Ok((true, rest)),
            _ => Err(DeserError::new::<bool>(input, DeserErrorKind::InvalidBool(byte), 0..0)),
        }
    }

    /// Deserialize a [`u8`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`u8`].
    fn deserialize_u8<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u8, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `u8`");

        match input.split_first() {
            Some((first, rest)) => Ok((*first, rest)),
            None => Err(DeserError::new::<u8>(input, DeserErrorKind::EndOfInput(Some(1)), 0..1)),
        }
    }

    /// Deserialize a [`i8`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`i8`].
    #[inline]
    #[expect(clippy::cast_possible_wrap, reason = "Wrapping is desired here")]
    fn deserialize_i8<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i8, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `i8`");

        self.deserialize_u8(input).map(|(v, rest)| (v as i8, rest))
    }

    /// Deserialize a [`u16`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`u16`].
    fn deserialize_u16<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u16, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `u16`");

        match input.split_first_chunk::<2>() {
            Some((first, rest)) => Ok((u16::from_be_bytes(*first), rest)),
            None => Err(DeserError::new::<u16>(
                input,
                DeserErrorKind::EndOfInput(Some(2 - input.len())),
                0..input.len(),
            )),
        }
    }

    /// Deserialize a variable-length [`u16`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`u16`].
    fn deserialize_var_u16<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u16, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `var_u16`");

        let mut cursor = input;
        let mut number: u16 = 0;

        for i in 0..3 {
            if let Some((&byte, remainder)) = cursor.split_first() {
                cursor = remainder;
                number |= u16::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                Err(DeserError::new::<u16>(
                    input,
                    DeserErrorKind::EndOfInput(Some(1)),
                    0..input.len(),
                ))?;
            }
        }

        Ok((number, cursor))
    }

    /// Deserialize a [`i16`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`i16`].
    #[inline]
    #[expect(clippy::cast_possible_wrap, reason = "Wrapping is desired here")]
    fn deserialize_i16<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i16, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `i16`");

        self.deserialize_u16(input)
            .map_or_else(|err| Err(err.with_type_name::<i16>()), |(v, rest)| Ok((v as i16, rest)))
    }

    /// Deserialize a variable-length [`i16`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`i16`].
    #[inline]
    #[expect(clippy::cast_possible_wrap, reason = "Wrapping is desired here")]
    fn deserialize_var_i16<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i16, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `var_i16`");

        self.deserialize_var_u16(input)
            .map_or_else(|err| Err(err.with_type_name::<i16>()), |(v, rest)| Ok((v as i16, rest)))
    }

    /// Deserialize a [`u32`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`u32`].
    fn deserialize_u32<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u32, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `u32`");

        match input.split_first_chunk::<4>() {
            Some((first, rest)) => Ok((u32::from_be_bytes(*first), rest)),
            None => Err(DeserError::new::<u32>(
                input,
                DeserErrorKind::EndOfInput(Some(4 - input.len())),
                0..input.len(),
            )),
        }
    }

    /// Deserialize a variable-length [`u32`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`u32`].
    fn deserialize_var_u32<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u32, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `var_u32`");

        let mut cursor = input;
        let mut number: u32 = 0;

        for i in 0..5 {
            if let Some((&byte, remainder)) = cursor.split_first() {
                cursor = remainder;
                number |= u32::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                Err(DeserError::new::<u32>(
                    input,
                    DeserErrorKind::EndOfInput(Some(1)),
                    0..input.len(),
                ))?;
            }
        }

        Ok((number, cursor))
    }

    /// Deserialize a [`i32`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`i32`].
    #[inline]
    #[expect(clippy::cast_possible_wrap, reason = "Wrapping is desired here")]
    fn deserialize_i32<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i32, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `i32`");

        self.deserialize_u32(input)
            .map_or_else(|err| Err(err.with_type_name::<i32>()), |(v, rest)| Ok((v as i32, rest)))
    }

    /// Deserialize a variable-length [`i32`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`i32`].
    #[inline]
    #[expect(clippy::cast_possible_wrap, reason = "Wrapping is desired here")]
    fn deserialize_var_i32<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i32, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `var_i32`");

        self.deserialize_var_u32(input)
            .map_or_else(|err| Err(err.with_type_name::<i32>()), |(v, rest)| Ok((v as i32, rest)))
    }

    /// Deserialize a [`u64`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`u64`].
    fn deserialize_u64<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u64, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `u64`");

        match input.split_first_chunk::<8>() {
            Some((first, rest)) => Ok((u64::from_be_bytes(*first), rest)),
            None => Err(DeserError::new::<u64>(
                input,
                DeserErrorKind::EndOfInput(Some(8 - input.len())),
                0..input.len(),
            )),
        }
    }

    /// Deserialize a variable-length [`u64`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`u64`].
    fn deserialize_var_u64<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u64, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `var_u64`");

        let mut cursor = input;
        let mut number: u64 = 0;

        for i in 0..10 {
            if let Some((&byte, remainder)) = cursor.split_first() {
                cursor = remainder;
                number |= u64::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                Err(DeserError::new::<u64>(
                    input,
                    DeserErrorKind::EndOfInput(Some(1)),
                    0..(input.len() - cursor.len()).saturating_sub(1),
                ))?;
            }
        }

        Ok((number, cursor))
    }

    /// Deserialize a [`i64`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`i64`].
    #[inline]
    #[expect(clippy::cast_possible_wrap, reason = "Wrapping is desired here")]
    fn deserialize_i64<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i64, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `i64`");

        self.deserialize_u64(input)
            .map_or_else(|err| Err(err.with_type_name::<i64>()), |(v, rest)| Ok((v as i64, rest)))
    }

    /// Deserialize a variable-length [`i64`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`i64`].
    #[inline]
    #[expect(clippy::cast_possible_wrap, reason = "Wrapping is desired here")]
    fn deserialize_var_i64<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i64, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `var_i64`");

        self.deserialize_var_u64(input)
            .map_or_else(|err| Err(err.with_type_name::<i64>()), |(v, rest)| Ok((v as i64, rest)))
    }

    /// Deserialize a [`u128`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`u128`].
    fn deserialize_u128<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u128, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `u128`");

        match input.split_first_chunk::<16>() {
            Some((first, rest)) => Ok((u128::from_be_bytes(*first), rest)),
            None => Err(DeserError::new::<u128>(
                input,
                DeserErrorKind::EndOfInput(Some(16 - input.len())),
                0..input.len(),
            )),
        }
    }

    /// Deserialize a variable-length [`u128`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`u128`].
    fn deserialize_var_u128<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(u128, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `var_u128`");

        let mut cursor = input;
        let mut number: u128 = 0;

        for i in 0..19 {
            if let Some((&byte, remainder)) = cursor.split_first() {
                cursor = remainder;
                number |= u128::from(byte & 0b0111_1111) << (7 * i);
                if byte & 0b1000_0000 == 0 {
                    break;
                }
            } else {
                Err(DeserError::new::<u128>(
                    input,
                    DeserErrorKind::EndOfInput(Some(1)),
                    0..input.len(),
                ))?;
            }
        }
        Ok((number, cursor))
    }

    /// Deserialize a [`i128`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`i128`].
    #[inline]
    #[expect(clippy::cast_possible_wrap, reason = "Wrapping is desired here")]
    fn deserialize_i128<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i128, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `i128`");

        self.deserialize_u128(input)
            .map_or_else(|err| Err(err.with_type_name::<i128>()), |(v, rest)| Ok((v as i128, rest)))
    }

    /// Deserialize a variable-length [`i128`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`i128`].
    #[inline]
    #[expect(clippy::cast_possible_wrap, reason = "Wrapping is desired here")]
    fn deserialize_var_i128<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(i128, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `var_i128`");

        self.deserialize_var_u128(input)
            .map_or_else(|err| Err(err.with_type_name::<i128>()), |(v, rest)| Ok((v as i128, rest)))
    }

    /// Deserialize a [`usize`] from the input byte slice.
    ///
    /// # Warning
    ///
    /// If you are using a 32-bit platform, this value will be truncated to 32
    /// bits!
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`usize`].
    #[inline]
    #[expect(clippy::cast_possible_truncation, reason = "That is the risk of using usize")]
    fn deserialize_usize<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(usize, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `usize`");

        self.deserialize_u64(input).map_or_else(
            |err| Err(err.with_type_name::<usize>()),
            |(v, rest)| Ok((v as usize, rest)),
        )
    }

    /// Deserialize a variable-length [`usize`] from the input byte slice.
    ///
    /// # Warning
    ///
    /// If you are using a 32-bit platform, this value will be truncated to 32
    /// bits!
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`usize`].
    #[inline]
    #[expect(clippy::cast_possible_truncation, reason = "That is the risk of using usize")]
    fn deserialize_var_usize<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(usize, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `var_usize`");

        self.deserialize_var_u64(input).map_or_else(
            |err| Err(err.with_type_name::<usize>()),
            |(v, rest)| Ok((v as usize, rest)),
        )
    }

    /// Deserialize a [`isize`] from the input byte slice.
    ///
    /// # Warning
    ///
    /// If you are using a 32-bit platform, this value will be truncated to 32
    /// bits!
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`isize`].
    #[inline]
    #[expect(clippy::cast_possible_truncation, reason = "That is the risk of using isize")]
    fn deserialize_isize<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(isize, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `isize`");

        self.deserialize_i64(input).map_or_else(
            |err| Err(err.with_type_name::<isize>()),
            |(v, rest)| Ok((v as isize, rest)),
        )
    }

    /// Deserialize a variable-length [`isize`] from the input byte slice.
    ///
    /// # Warning
    ///
    /// If you are using a 32-bit platform, this value will be truncated to 32
    /// bits!
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// variable-length [`isize`].
    #[inline]
    #[expect(clippy::cast_possible_truncation, reason = "That is the risk of using isize")]
    fn deserialize_var_isize<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(isize, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `var_isize`");

        self.deserialize_var_i64(input).map_or_else(
            |err| Err(err.with_type_name::<isize>()),
            |(v, rest)| Ok((v as isize, rest)),
        )
    }

    /// Deserialize a [`f32`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`f32`].
    #[inline]
    fn deserialize_f32<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(f32, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `f32`");

        self.deserialize_u32(input).map_or_else(
            |err| Err(err.with_type_name::<f32>()),
            |(v, rest)| Ok((f32::from_bits(v), rest)),
        )
    }

    /// Deserialize a [`f64`] from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid [`f64`].
    #[inline]
    fn deserialize_f64<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(f64, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `f64`");

        self.deserialize_u64(input).map_or_else(
            |err| Err(err.with_type_name::<f64>()),
            |(v, rest)| Ok((f64::from_bits(v), rest)),
        )
    }

    /// Deserialize a length-prefixed byte slice from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// length-prefix, or if there are not enough bytes in the input to
    /// satisfy the length.
    fn deserialize_bytes<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(&'input [u8], &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `&[u8]`");

        let (len, rest) =
            self.deserialize_var_usize(input).map_err(DeserError::with_type_name::<&[u8]>)?;

        if let Some((bytes, rest)) = rest.split_at_checked(len) {
            Ok((bytes, rest))
        } else {
            #[allow(unused_mut, reason = "Only used when rich diagnostics are enabled")]
            let mut error = DeserError::new::<&[u8]>(
                input,
                DeserErrorKind::EndOfInput(Some(len - rest.len())),
                input.len() - rest.len()..input.len(),
            );

            #[cfg(feature = "rich-diagnostics")]
            {
                error = error.with_label(
                    format!("Array prefix expects {len} bytes"),
                    0..(input.len() - rest.len()).saturating_sub(1),
                );
            }

            Err(error)
        }
    }

    /// Deserialize a length-prefixed UTF-8 string from the input byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the next bytes do not represent a valid
    /// length-prefix, if there are not enough bytes in the input to
    /// satisfy the length, or if the bytes are not valid UTF-8.
    fn deserialize_str<'input>(
        &mut self,
        input: &'input [u8],
    ) -> Result<(&'input str, &'input [u8]), DeserError<'input>> {
        #[cfg(feature = "trace")]
        tracing::trace!("Deserializing `&str`");

        let (bytes, rest) =
            self.deserialize_bytes(input).map_err(DeserError::with_type_name::<&str>)?;

        match core::str::from_utf8(bytes) {
            Ok(str) => Ok((str, rest)),
            #[allow(unused_mut, reason = "Only used when rich diagnostics are enabled")]
            Err(err) => {
                let mut error = DeserError::new::<&str>(
                    input,
                    DeserErrorKind::InvalidUtf8(err),
                    input.len() - bytes.len() + err.valid_up_to()
                        ..if let Some(size) = err.error_len() {
                            (input.len() - bytes.len() + err.valid_up_to() + size).saturating_sub(1)
                        } else {
                            (input.len() - rest.len()).saturating_sub(1)
                        },
                );

                #[cfg(feature = "rich-diagnostics")]
                {
                    error = error
                        .with_label(
                            "While reading this string",
                            input.len() - rest.len() - bytes.len()
                                ..(input.len() - bytes.len()).saturating_sub(1) + err.valid_up_to(),
                        )
                        .with_note(format!(
                            "Valid string contains: \"{}\"",
                            core::str::from_utf8(&bytes[..err.valid_up_to()]).unwrap()
                        ));
                }

                Err(error)
            }
        }
    }
}
