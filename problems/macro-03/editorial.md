One rule can expand to several items: the enum, an inherent `impl` and two trait impls. Inside them,
repetitions write one piece per variant: `&[$(Self::$variant),*]` for `ALL`, and one match arm per
variant in `name`. A pattern must name the variant by its path, `Self::$variant`. A bare `$variant`
would be a new variable that matches anything, which rustc rejects (E0170), and for the tests'
variant `None`, it would be the `None` of `Option`.

`$(...)?` has no "else" branch for a variant without a label. The solution forwards `$($label)?` to
internal rules, which receive either `@name Red` or `@name North "N"` and have one rule for each
case. `stringify!($variant)` turns the identifier into the string literal `"Red"`. In the
transcriber, `$label` must stay inside `$(...)?`: it was matched inside two repetitions, so a bare
`$label` in the arm gives "variable `label` is still repeating at this depth".

The internal call is where `#[macro_export]` matters. It puts the macro at the root of the crate,
so the call is `$crate::named_enum!`, not `$crate::user::named_enum!` (the macro is not an item of
`user`), and not a bare `named_enum!`, which is not in scope in the tests' modules ("cannot find
macro `named_enum` in this scope"). For the same reason, `$crate` is the crate root, and the given
type is `$crate::user::UnknownName`, where it is defined.

Every other path in the expansion is resolved where the macro is called. The tests import nothing
from the standard library, so `fmt::Display` is not found ("cannot find module or crate `fmt` in
this scope"), and `Result<Self, Self::Err>` finds the tests' own one-parameter alias ("type alias
takes 1 generic argument but 2 generic arguments were supplied"). Absolute paths such as
`::std::fmt::Display` and `::std::result::Result` work in any module.

The generated items also become items of the caller's module. The tests call the macro several times
in one module, so a `use std::fmt;` or a helper function with a fixed name in the expansion is
defined twice (E0252, E0428). Impls for different types do not clash, so keep helpers inside them
(or see the alternatives).

An enum without variants is allowed. `match self {}` compiles in `name(self)`, because `self` is a
value of a type with no values. But `Display::fmt` gets `&self`, and a reference is always
considered to have values: `match self {}` there gives "non-exhaustive patterns: type `&Never` is
non-empty". Calling `self.name()` copies the value and reuses the match in `name`
(`match *self { ... }` would also work). Likewise, `from_name` reuses `ALL` and `name()`, so labels
are handled in one place, and `from_str` reuses `from_name`.

### Alternatives

Without internal rules, an array literal picks the label: with a label, the arm is
`["N", "North"][0]`, and without one, `["Red"][0]`.

```rust
pub fn name(self) -> &'static str {
    match self {
        $(Self::$variant => [$($label,)? ::std::stringify!($variant)][0],)*
    }
}
```

`from_name` can be a `match` on the string, too. A macro call may be used as a pattern, and the
internal rules expand to string literals.

```rust
pub fn from_name(name: &str) -> ::std::option::Option<Self> {
    match name {
        $($crate::named_enum!(@name $variant $($label)?) => {
            ::std::option::Option::Some(Self::$variant)
        })*
        _ => ::std::option::Option::None,
    }
}
```

An unnamed constant `const _` can be defined any number of times in a module, and the `use`
declarations in its block are local to it. The impls inside apply everywhere, and the local
`use ::std::result::Result;` hides the caller's alias.

```rust
const _: () = {
    use ::std::fmt::{self, Display, Formatter};
    use ::std::result::Result;
    use ::std::str::FromStr;
    use $crate::user::UnknownName;

    impl Display for $name {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str(self.name())
        }
    }

    impl FromStr for $name {
        type Err = UnknownName;

        fn from_str(s: &str) -> Result<Self, UnknownName> {
            Self::from_name(s).ok_or_else(|| UnknownName(s.to_string()))
        }
    }
};
```
