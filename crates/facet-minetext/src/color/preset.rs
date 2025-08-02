//! Preset color definitions

pub mod owo_colors {
    //! Re-exports from the [`owo_colors`] crate.
    pub use ::owo_colors::{
        BgColorDisplay, BgDynColorDisplay, Color, DynColor, FgColorDisplay, FgDynColorDisplay, Rgb,
        colors::CustomColor,
    };
}

macro_rules! generate_colors {
    ($($color:ident $name:literal $char:literal $fg:literal $bg:literal),* $(,)?) => {

        /// An enum representing all of the named colors.
        #[repr(u8)]
        #[expect(missing_docs)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "facet", derive(facet_macros::Facet))]
        pub enum MineColors {
            $(
                #[cfg_attr(feature = "facet", facet(rename = $name))]
                $color,
            )*
        }

        impl<'a> From<&'a str> for MineColors {
            fn from(name: &'a str) -> Self {
                match name {
                    $($color::NAME => Self::$color,)*
                    _ => Self::White, // Default to white if no match
                }
            }
        }

        impl From<char> for MineColors {
            fn from(c: char) -> Self {
                match c {
                    $($color::CHAR => Self::$color,)*
                    _ => Self::White, // Default to white if no match
                }
            }
        }

        impl MineColors {
            /// Get the name of the color.
            #[must_use]
            pub const fn name(&self) -> &'static str {
                match self {
                    $(MineColors::$color => $color::NAME,)*
                }
            }

            /// Get the [`char`] used to represent the color.
            #[must_use]
            pub const fn char(&self) -> char {
                match self {
                    $(MineColors::$color => $color::CHAR,)*
                }
            }

            /// Get the foreground color as a [`DynColors`](::owo_colors::DynColors).
            #[must_use]
            pub const fn fg(&self) -> ::owo_colors::DynColors {
                match self {
                    $(MineColors::$color => $color::FG_DYNCOLOR,)*
                }
            }
            /// Get the background color as a [`DynColors`](::owo_colors::DynColors).
            #[must_use]
            pub const fn bg(&self) -> ::owo_colors::DynColors {
                match self {
                    $(MineColors::$color => $color::BG_DYNCOLOR,)*
                }
            }

            /// Get the foreground color as a [`u32`].
            #[must_use]
            pub const fn fg_u32(&self) -> u32 {
                match self {
                    $(MineColors::$color => $color::FG_U32,)*
                }
            }
            /// Get the background color as a [`u32`].
            #[must_use]
            pub const fn bg_u32(&self) -> u32 {
                match self {
                    $(MineColors::$color => $color::BG_U32,)*
                }
            }
        }

        $(
            #[expect(missing_docs)]
            pub struct $color;

            impl private::Sealed for $color {}
            impl MineColor for $color {
                const NAME: &'static str = $name;
                const CHAR: char = $char;

                const FG_U32: u32 = const_panic::unwrap_ok!(u32::from_str_radix($fg, 16));
                const FG_DYNCOLOR: ::owo_colors::DynColors = <Self::Foreground as ::owo_colors::Color>::DYN_COLORS_EQUIVALENT;
                type Foreground = ::owo_colors::colors::CustomColor<{((Self::FG_U32 >> 16) & 0x0000FF) as u8 }, {((Self::FG_U32 >> 8) & 0x0000FF) as u8}, {(Self::FG_U32 & 0x0000FF) as u8}>;

                const BG_U32: u32 = const_panic::unwrap_ok!(u32::from_str_radix($bg, 16));
                const BG_DYNCOLOR: ::owo_colors::DynColors = <Self::Background as ::owo_colors::Color>::DYN_COLORS_EQUIVALENT;
                type Background = ::owo_colors::colors::CustomColor<{((Self::BG_U32 >> 16) & 0x0000FF) as u8}, {((Self::BG_U32 >> 8) & 0x0000FF) as u8}, {(Self::BG_U32 & 0x0000FF) as u8}>;
            }

            impl From<$color> for MineColors {
                fn from(_: $color) -> Self { MineColors::$color }
            }
        )*

        #[test]
        #[cfg(feature = "std")]
        fn print() {
            use ::owo_colors::{OwoColorize, DynColors};

            const NAMES: &[&str] = &[$(stringify!($color)),*];
            const FG_LIST: &[DynColors] = &[$(MineColors::$color.fg()),*];
            const BG_LIST: &[DynColors] = &[$(MineColors::$color.bg()),*];

            for ((name, fg), bg) in NAMES.iter().zip(FG_LIST.iter()).zip(BG_LIST.iter()) {
                let foreground = std::format!("Foreground {fg:?}");
                let background = std::format!("Background {bg:?}");
                let space = " ".repeat(14 - name.len());

                std::println!("{name}:{space}{}{}{}",
                    foreground.color(*fg).on_color(MineColors::Black.bg()),
                    " ".repeat(45 - space.len() - name.len() - foreground.len()),
                    background.color(MineColors::White.fg()).on_color(*bg),
                );
            }
        }
    };
}

generate_colors! {
    Black       "black"        '0' "000000" "000000",
    DarkBlue    "dark_blue"    '1' "0000AA" "00002A",
    DarkGreen   "dark_green"   '2' "00AA00" "002A00",
    DarkAqua    "dark_aqua"    '3' "00AAAA" "002A2A",
    DarkRed     "dark_red"     '4' "AA0000" "2A0000",
    DarkPurple  "dark_purple"  '5' "AA00AA" "2A002A",
    Gold        "gold"         '6' "FFAA00" "3E2A00",
    Gray        "gray"         '7' "AAAAAA" "2A2A2A",
    DarkGray    "dark_gray"    '8' "555555" "151515",
    Blue        "blue"         '9' "5555FF" "15153F",
    Green       "green"        'a' "55FF55" "153F15",
    Aqua        "aqua"         'b' "55FFFF" "153F3F",
    Red         "red"          'c' "FF5555" "3F1515",
    LightPurple "light_purple" 'd' "FF55FF" "3F153F",
    Yellow      "yellow"       'e' "FFFF55" "3F3F15",
    White       "white"        'f' "FFFFFF" "3F3F3F",
}

/// A trait for defining foreground and background colors.
pub trait MineColor: private::Sealed {
    /// The name of the color.
    const NAME: &'static str;
    /// The character used to represent the color.
    const CHAR: char;

    /// The foreground color type.
    type Foreground: ::owo_colors::Color;
    /// The foreground color as a [`u32`].
    const FG_U32: u32;
    /// The foreground color as a [`DynColors`](::owo_colors::DynColors).
    const FG_DYNCOLOR: ::owo_colors::DynColors;

    /// The background color type.
    type Background: ::owo_colors::Color;
    /// The background color as a [`u32`].
    const BG_U32: u32;
    /// The background color as a [`DynColors`](::owo_colors::DynColors).
    const BG_DYNCOLOR: ::owo_colors::DynColors;
}

#[rustfmt::skip]
mod private { #[doc(hidden)] pub trait Sealed {} }

// -------------------------------------------------------------------------------------------------

/// A wrapper around [`OwoColorize`](::owo_colors::OwoColorize)
/// specifically for [`MineColors`].
pub trait MineColorize: ::owo_colors::OwoColorize {
    /// Set the foreground color generically.
    ///
    /// ---
    ///
    /// See [`OwoColorize::fg`](::owo_colors::OwoColorize::fg) for more details.
    #[inline]
    fn fg<C: MineColor>(
        &self,
    ) -> ::owo_colors::FgColorDisplay<'_, <C as MineColor>::Foreground, Self> {
        <Self as ::owo_colors::OwoColorize>::fg::<<C as MineColor>::Foreground>(&self)
    }

    /// Set the background color generically.
    ///
    /// ---
    ///
    /// See [`OwoColorize::bg`](::owo_colors::OwoColorize::bg) for more details.
    #[inline]
    fn bg<C: MineColor>(
        &self,
    ) -> ::owo_colors::BgColorDisplay<'_, <C as MineColor>::Background, Self> {
        <Self as ::owo_colors::OwoColorize>::bg::<<C as MineColor>::Background>(&self)
    }

    /// Set the foreground color at runtime.
    ///
    /// Only use if you do not know which color will be used at compile-time.
    /// If the color is constant, use [`MineColorize::fg`] instead.
    ///
    /// ---
    ///
    /// See [`OwoColorize::color`](::owo_colors::OwoColorize::color) for more
    /// details.
    fn fg_using<C: Into<MineColors>>(
        &self,
        color: C,
    ) -> ::owo_colors::FgDynColorDisplay<'_, ::owo_colors::DynColors, Self> {
        <Self as ::owo_colors::OwoColorize>::color(self, color.into().fg())
    }

    /// Set the background color at runtime.
    ///
    /// Only use if you do not know which color will be used at compile-time.
    /// If the color is constant, use [`MineColorize::bg`] instead.
    ///
    /// ---
    ///
    /// See [`OwoColorize::on_color`](::owo_colors::OwoColorize::on_color) for
    /// more details.
    fn bg_using<C: Into<MineColors>>(
        &self,
        color: C,
    ) -> ::owo_colors::BgDynColorDisplay<'_, ::owo_colors::DynColors, Self> {
        <Self as ::owo_colors::OwoColorize>::on_color(self, color.into().bg())
    }
}

impl<T: ::owo_colors::OwoColorize> MineColorize for T {}
