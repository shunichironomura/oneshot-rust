use crate::judge::{case, check, check_eq};

// Like many modules, this one has its own `Result`, and it imports nothing else that the
// expansion of `named_enum!` could rely on.
type Result<T> = std::result::Result<T, String>;

crate::named_enum! { Color { Red, Green, Blue } }
crate::named_enum! { Dir { North = "N", East = "E", South = "S", West = "W", } }
crate::named_enum! { Unit { Only, } }
crate::named_enum! { Never {} }
crate::named_enum! { Swap { A = "B", B = "A", C } }
crate::named_enum! {
    Fruit {
        Apple = "りんご",
        Banana,
        Cherry = "cherry pie",
        Blank = "",
    }
}
crate::named_enum! { Level { None, Low, High = "HIGH" } }

mod inner {
    // The generated items are used from the parent module.
    crate::named_enum! { Coin { Heads, Tails = "tails" } }
}

use inner::Coin;

/// Stand-ins that let this file compile while the macro does not generate everything yet, as in
/// the starting code. The items that the macro generates take precedence over them, as long as the
/// tests call `name` on values, not references. They are only in scope inside `run`, never where
/// the macro is called: there, `Missing::name(&self)` would win over a generated `name(self)` for
/// a call like `self.name()` on a `&Self`.
mod support {
    use std::marker::PhantomData;
    use std::str::FromStr;

    use crate::user::UnknownName;

    pub trait Missing: Sized + 'static {
        const ALL: &'static [Self] = &[];

        fn name(&self) -> &'static str {
            "(no name method)"
        }

        fn from_name(_: &str) -> Option<Self> {
            None
        }
    }

    impl<T: 'static> Missing for T {}

    pub struct Shown<'a, T>(pub &'a T);

    pub trait WithDisplay {
        fn shown(&self) -> Option<String>;
    }

    impl<T: std::fmt::Display> WithDisplay for Shown<'_, T> {
        fn shown(&self) -> Option<String> {
            Some(self.0.to_string())
        }
    }

    pub trait WithoutDisplay {
        fn shown(&self) -> Option<String>;
    }

    impl<T> WithoutDisplay for &Shown<'_, T> {
        fn shown(&self) -> Option<String> {
            None
        }
    }

    pub struct Parsed<T>(pub PhantomData<T>);

    pub trait WithFromStr<T> {
        fn parsed(&self, s: &str) -> Option<Result<T, UnknownName>>;
    }

    impl<T: FromStr<Err = UnknownName>> WithFromStr<T> for Parsed<T> {
        fn parsed(&self, s: &str) -> Option<Result<T, UnknownName>> {
            Some(s.parse())
        }
    }

    pub trait WithoutFromStr<T> {
        fn parsed(&self, s: &str) -> Option<Result<T, UnknownName>>;
    }

    impl<T> WithoutFromStr<T> for &Parsed<T> {
        fn parsed(&self, _: &str) -> Option<Result<T, UnknownName>> {
            None
        }
    }
}

/// `Some(value.to_string())`, or `None` if the type does not implement `Display`.
macro_rules! shown {
    ($value:expr) => {{
        use support::{WithDisplay as _, WithoutDisplay as _};
        (&support::Shown(&$value)).shown()
    }};
}

/// `Some(s.parse::<T>())`, or `None` if `T` does not implement `FromStr<Err = UnknownName>`.
macro_rules! parsed {
    ($t:ty, $s:expr) => {{
        use support::{WithFromStr as _, WithoutFromStr as _};
        (&support::Parsed::<$t>(std::marker::PhantomData)).parsed($s)
    }};
}

/// Every variant of each enum is found again by its name, with `from_name` and with `parse`.
macro_rules! check_round_trips {
    ($($t:ty),*) => {{
        $(
            for &v in <$t>::ALL {
                check_eq(<$t>::from_name(v.name()), Some(v))?;
                parses(parsed!($t, v.name()), Ok(v))?;
            }
        )*
    }};
}

fn unknown(s: &str) -> crate::user::UnknownName {
    crate::user::UnknownName(s.to_string())
}

fn displays(got: Option<String>, expected: &str) -> Result<()> {
    match got {
        Some(text) => check_eq(text.as_str(), expected),
        None => Err("the enum does not implement Display".to_string()),
    }
}

fn parses<T: PartialEq + std::fmt::Debug>(
    got: Option<std::result::Result<T, crate::user::UnknownName>>,
    expected: std::result::Result<T, crate::user::UnknownName>,
) -> Result<()> {
    match got {
        Some(result) => check_eq(result, expected),
        None => Err("the enum does not implement FromStr with Err = UnknownName".to_string()),
    }
}

pub fn run() {
    // Only used by the starting code; see `support`.
    use support::Missing as _;

    case("sample-1", || {
        check_eq(Color::ALL, &[Color::Red, Color::Green, Color::Blue][..])?;
        check_eq(Color::Green.name(), "Green")?;
        check_eq(Color::from_name("Blue"), Some(Color::Blue))?;
        check_eq(Color::from_name("blue"), None)?;
        displays(shown!(Color::Red), "Red")?;
        parses(parsed!(Color, "Green"), Ok(Color::Green))?;
        parses(parsed!(Color, "Purple"), Err(unknown("Purple")))
    });
    case("sample-2", || {
        check_eq(Dir::ALL, &[Dir::North, Dir::East, Dir::South, Dir::West][..])?;
        check_eq(Dir::West.name(), "W")?;
        check_eq(Dir::from_name("W"), Some(Dir::West))?;
        check_eq(Dir::from_name("West"), None)?;
        displays(shown!(Dir::East), "E")?;
        parses(parsed!(Dir, "S"), Ok(Dir::South))?;
        parses(parsed!(Dir, "South"), Err(unknown("South")))
    });
    case("hidden-1", || {
        // One variant and a trailing comma. `ALL` is a `'static` constant, and names outlive
        // the strings that `from_name` is called with.
        const UNITS: &[Unit] = Unit::ALL;
        check_eq(UNITS, &[Unit::Only][..])?;
        let name: &'static str = Unit::Only.name();
        check_eq(name, "Only")?;
        check_eq(Unit::from_name(&String::from("Only")), Some(Unit::Only))?;
        displays(shown!(Unit::Only), "Only")?;
        parses(parsed!(Unit, "Only"), Ok(Unit::Only))
    });
    case("hidden-2", || {
        // Only exact matches count, and the error keeps the string as it was.
        for s in ["red", "RED", " Red", "Red ", "Re", "", "Color::Red"] {
            check_eq(Color::from_name(s), None)?;
        }
        for s in ["n", "North", " N", "NE", ""] {
            check_eq(Dir::from_name(s), None)?;
        }
        check_eq(Unit::from_name("Unit"), None)?;
        parses(parsed!(Color, " Red"), Err(unknown(" Red")))?;
        parses(parsed!(Color, "red"), Err(unknown("red")))?;
        parses(parsed!(Dir, "n"), Err(unknown("n")))
    });
    case("hidden-3", || {
        // An enum without variants.
        check_eq(Never::ALL.len(), 0)?;
        check_eq(Never::from_name(""), None)?;
        check_eq(Never::from_name("Never"), None)?;
        parses(parsed!(Never, "Never"), Err(unknown("Never")))
    });
    case("hidden-4", || {
        // A label replaces the identifier, even when it is the identifier of another variant.
        check_eq(Swap::ALL, &[Swap::A, Swap::B, Swap::C][..])?;
        check_eq([Swap::A.name(), Swap::B.name(), Swap::C.name()], ["B", "A", "C"])?;
        check_eq(Swap::from_name("A"), Some(Swap::B))?;
        check_eq(Swap::from_name("B"), Some(Swap::A))?;
        check_eq(Swap::from_name("C"), Some(Swap::C))?;
        displays(shown!(Swap::A), "B")?;
        parses(parsed!(Swap, "A"), Ok(Swap::B))?;
        parses(parsed!(Swap, "B"), Ok(Swap::A))
    });
    case("hidden-5", || {
        // Labels with non-ASCII characters and spaces, and an empty label.
        check_eq(Fruit::ALL, &[Fruit::Apple, Fruit::Banana, Fruit::Cherry, Fruit::Blank][..])?;
        let names = [Fruit::Apple, Fruit::Banana, Fruit::Cherry, Fruit::Blank].map(|f| f.name());
        check_eq(names, ["りんご", "Banana", "cherry pie", ""])?;
        check_eq(Fruit::from_name("りんご"), Some(Fruit::Apple))?;
        check_eq(Fruit::from_name("Apple"), None)?;
        check_eq(Fruit::from_name("cherry"), None)?;
        check_eq(Fruit::from_name(""), Some(Fruit::Blank))?;
        displays(shown!(Fruit::Cherry), "cherry pie")?;
        displays(shown!(Fruit::Blank), "")?;
        parses(parsed!(Fruit, ""), Ok(Fruit::Blank))?;
        parses(parsed!(Fruit, "Blank"), Err(unknown("Blank")))
    });
    case("hidden-6", || {
        // A variant named `None`, and an enum defined in another module.
        check_eq(Level::ALL, &[Level::None, Level::Low, Level::High][..])?;
        let names = [Level::None, Level::Low, Level::High].map(|level| level.name());
        check_eq(names, ["None", "Low", "HIGH"])?;
        check_eq(Level::from_name("None"), Some(Level::None))?;
        check_eq(Level::from_name("High"), None)?;
        parses(parsed!(Level, "HIGH"), Ok(Level::High))?;
        check_eq(Coin::ALL, &[Coin::Heads, Coin::Tails][..])?;
        check_eq([Coin::Heads.name(), Coin::Tails.name()], ["Heads", "tails"])?;
        check_eq(Coin::from_name("tails"), Some(Coin::Tails))?;
        check_eq(Coin::from_name("Tails"), None)?;
        displays(shown!(Coin::Tails), "tails")?;
        parses(parsed!(Coin, "Heads"), Ok(Coin::Heads))?;
        parses(parsed!(Coin, "Tails"), Err(unknown("Tails")))
    });
    case("hidden-7", || {
        check_round_trips!(Color, Dir, Unit, Swap, Fruit, Level, Coin);
        let lengths = [
            Color::ALL.len(),
            Dir::ALL.len(),
            Unit::ALL.len(),
            Swap::ALL.len(),
            Fruit::ALL.len(),
            Level::ALL.len(),
            Coin::ALL.len(),
        ];
        check(lengths == [3, 4, 1, 3, 4, 3, 2], "every variant is in ALL exactly once")
    });
}
