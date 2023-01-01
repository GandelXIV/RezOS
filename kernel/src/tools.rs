/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! This module contains various utility functions and macros used everywhere


/// implements Into<&str> for a public enum that converts the name of a variant into a string.
///
/// # Example
/// ```
/// enum_names! {
///     pub enum Sum {
///         Foo,
///         Bar,
///     }
/// }
///
/// assert_eq!(Sum::Foo.into(), "Foo");
/// assert_eq!(Sum::Bar.into(), "Bar");
/// ```
#[macro_export]
#[macro_use]
macro_rules! enum_names {
    (pub enum $name:ident {
        $($variant:ident),*,
    }) => {
        pub enum $name {
            $($variant),*
        }

        impl Into<&'static str> for $name {
            fn into(self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

// use in an impl block, will create functions to interact with ranges of the raw binary represenation of the type
// Example usage:
/*
    impl u64 {
        bitfield!(set_half, u32, 31, 0); // creates a set_half() function that takes an u32 and writes to bits 0..31 of the u64
        bitfield!(set_flag, bool, 60); // creates a set_flag() function that takes a bool and writes to bit 60 of the u64
    }
*/
/// Generates functions to mutate bitranges in the binary of a type
/// Use this inside an `impl` block
///
/// # examples
/// ```
/// struct Token(u64);
/// impl Token {
///     bitfield!(set_foo, u16, 14, 0); // generates a set_foo function that takes an u16 and writes it into bits 0-14
///     bitfield!(set_flag, get_flag, bool, 17); // generates a set_flag function that takes a boolean and writes to bit 17
///     bitfield!(set_bar, get_bar, u32, 52, 20); // generates a set_flag and get_flag function that operate on bits 20-52
///
///     fn flip_flag(&mut self) {
///         self.set_flag( !self.get_flag() )
///     }
///     ...
/// }
/// ```
#[macro_export]
#[macro_use]
macro_rules! bitfield {
    // bitrange, only setter
    ($name:ident, $typ:ty, $h:literal, $l:literal) => {
        const fn $name(&mut self, payload: $typ) {
            *self = Self(bin_insert(self.0, payload, $h, $l));
        }
    };

    // bitrange, setter & getter
    ($set_name:ident, $get_name:ident, $typ:ty, $h:literal, $l:literal) => {
        const fn $set_name(&mut self, payload: $typ) {
            *self = Self(bin_insert(self.0, payload, $h, $l));
        }

        const fn $get_name(&mut self) -> $typ {
            bin_extract(self.0, $h, $l).into()
        }
    };
    
    // single-bit, only setter
    ($name:ident, $typ:ty, $b:literal) => {
        const fn $name(&mut self, payload: $typ) {
            *self = Self(bin_insert(self.0, payload, $b, $b));
        }
    };

    // single bit, setter & getter
    ($set_name:ident, $get_name:ident, $typ:ty, $b:literal) => {
        const fn $set_name(&mut self, payload: $typ) {
            *self = Self(bin_insert(self.0, payload, $b, $b));
        }

        const fn $get_name(&self) -> $typ {
            bin_extract(self.0, $b, $b).into()
        }
    };
}


/// reads bits from a binary value 
///
/// # examples
/// ```
/// assert_eq!(bin_extract(0b11110011, 4, 1), 0b1001)
/// ```

pub const fn bin_extract<T: Into<u64>>(target: T, higher: usize, lower: usize) -> u64
where
    u64: ~const From<T>,
{
    // (x >> lower) -> cuts out the stuff we dont care about to the right
    // ( (1 << (higher + 1 - lower)) - 1) -> creates a binary number full of ones with length (higher+1-lower), that then get used as a mask
    // for bitwise and ('&') to remove all the stuff we dont care about to the left
    (target.into() >> lower) & ((1 << (higher + 1 - lower)) - 1)
}

/// inserts a binary sequence into another one
///
/// # examples
/// ```
/// assert_eq!(bin_insert(0b0000000000000000, 0b00001111, 6, 3), 0b0000000000111100)
/// assert_eq!(bin_insert(0b1110001010001100, 0b101, 16, 14), 0b1010001010001100)
/// ```
pub const fn bin_insert<T: Into<u64>, U: Into<u64>>(
    target: T,
    payload: U,
    higher: usize,
    lower: usize,
) -> u64
where
    u64: ~const From<U>,
    u64: ~const From<T>,
{
    // this mask enables all bits that have to be with a BitOr
    let enable = (payload.into() << lower) as u64;
    // this mask disables all bits that must not be with a BitAnd
    // same as the enable mask, but full of ones on the left & the right to preserve the rest of
    // the data, except for our payload that can have some zeroes for disabling
    let disable = (enable | (((1 as u64) << lower) - 1))
        // overflow check
        | (if higher == 63 {
            0
        } else {
            u64::MAX << higher + 1
        });
    // apply the masks
    ((target.into() | enable) & disable).into()
}
