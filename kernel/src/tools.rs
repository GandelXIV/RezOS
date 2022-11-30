// Contains useful macros and utils that can be used anywhere in the kernel

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

// NOTE: The following 2 functions do NOT bound check if the payload fits

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
