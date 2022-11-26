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

// cuts out bits 'higher' to 'lower' from number 'x' as binary
// TODO: rewrite this using generics
pub const fn bin_extract(x: u32, higher: usize, lower: usize) -> u32 {
    // (x >> lower) -> cuts out the stuff we dont care about to the right
    // ( (1 << (higher + 1 - lower)) - 1) -> creates a binary number full of ones with length (higher+1-lower), that then get used as a mask
    // for bitwise and ('&') to remove all the stuff we dont care about to the left
    (x >> lower) & ( (1 << (higher + 1 - lower)) - 1)
}

