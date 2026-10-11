/// The error of `str::parse` for the generated enums: the string that is not a name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownName(pub String);

// named_enum! { Color { Red, Green = "green", Blue } } defines the enum `Color`,
// its methods and its trait impls. For now, the rule below only defines the enum.
#[macro_export]
macro_rules! named_enum {
    ($name:ident { $($variant:ident $(= $label:literal)?),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant),*
        }
    };
}
