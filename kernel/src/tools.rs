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
