/// The error of `str::parse` for the generated enums: the string that is not a name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownName(pub String);

// named_enum! { Color { Red, Green = "green", Blue } } defines the enum `Color`,
// its methods and its trait impls.
#[macro_export]
macro_rules! named_enum {
    ($name:ident { $($variant:ident $(= $label:literal)?),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub const ALL: &'static [Self] = &[$(Self::$variant),*];

            pub fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $crate::named_enum!(@name $variant $($label)?),)*
                }
            }

            pub fn from_name(name: &str) -> ::std::option::Option<Self> {
                Self::ALL.iter().copied().find(|v| v.name() == name)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.name())
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::user::UnknownName;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                Self::from_name(s)
                    .ok_or_else(|| $crate::user::UnknownName(::std::string::String::from(s)))
            }
        }
    };
    // Internal rules: the name of a variant without a label, and with one.
    (@name $variant:ident) => {
        ::std::stringify!($variant)
    };
    (@name $variant:ident $label:literal) => {
        $label
    };
}
