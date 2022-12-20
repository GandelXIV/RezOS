/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// implements Into<str> for any empty variant of a public enum
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
#[macro_export]
#[macro_use]
macro_rules! bitfield {
    // setters
    ($name:ident, $typ:ty, $h:literal, $l:literal) => {
        const fn $name(&mut self, payload: $typ) {
            *self = Self(bin_insert(self.0, payload, $h, $l));
        }
    };

    ($name:ident, $typ:ty, $b:literal) => {
        const fn $name(&mut self, payload: $typ) {
            *self = Self(bin_insert(self.0, payload, $b, $b));
        }
    };
}

// NOTE: The following 2 functions do NOT bound check the payload fits

// cuts out bits 'higher' to 'lower' from number 'x' as binary
pub const fn bin_extract<T: Into<u64>>(target: T, higher: usize, lower: usize) -> u64
where
    u64: ~const From<T>,
{
    // (x >> lower) -> cuts out the stuff we dont care about to the right
    // ( (1 << (higher + 1 - lower)) - 1) -> creates a binary number full of ones with length (higher+1-lower), that then get used as a mask
    // for bitwise and ('&') to remove all the stuff we dont care about to the left
    (target.into() >> lower) & ((1 << (higher + 1 - lower)) - 1)
}

// inserts a binary sequence into another one
// WARNING: does not check if 'payload' fits into the range higher<->lower
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
