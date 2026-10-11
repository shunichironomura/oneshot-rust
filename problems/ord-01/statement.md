Software versions such as `1.10.2` or `2.0.0-rc.1` are compared part by part, and comparing them
as strings gives wrong answers. Implement parsing, display and ordering for the type `Version`
below (it is already in the editor, together with `ParseVersionError`), and a function that finds
the newest stable version.

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVersionError;

impl FromStr for Version { /* type Err = ParseVersionError; */ }
impl fmt::Display for Version { /* to implement */ }
impl Ord for Version { /* to implement */ }
impl PartialOrd for Version { /* to implement */ }

pub fn latest_stable(versions: &[Version]) -> Option<&Version>
```

- **Parsing** (`from_str`, which `s.parse::<Version>()` uses): a valid string has the form
  `MAJOR[.MINOR[.PATCH]][-PRE]`, where the parts in brackets are optional.
  - `MAJOR`, `MINOR` and `PATCH` are each one or more ASCII digits (`0` to `9`), and their value
    must fit in a `u64`. Leading zeros are allowed: `007` is 7. A missing `MINOR` or `PATCH` is 0,
    and without `-PRE` the field `pre` is `None`.
  - `PRE`, the pre-release label, is one or more characters, each of which is an ASCII letter, an
    ASCII digit or `.`. It is stored in `pre` exactly as written.
  - Nothing else is allowed. The string is not trimmed, and any other character or any empty part
    makes it invalid. For an invalid string, return `Err(ParseVersionError)`. For example, `""`,
    `"1..2"`, `"1.2.3.4"`, `"1.2-"` and `"v1"` are all invalid.
- **Display** prints `MAJOR.MINOR.PATCH` as decimal numbers without leading zeros, followed by
  `-PRE` if `pre` is `Some`. For example, both `"1.2"` and `"01.2.0"` are displayed as `1.2.0`.
- **Order**:
  - Versions are compared by `major`, then by `minor`, then by `patch`, as numbers.
  - If all three are equal, a version without a pre-release is **greater** than one with a
    pre-release: `1.0.0-rc.1` < `1.0.0`.
  - If both have a pre-release, the labels are compared as strings, byte by byte (the order of `<`
    on `str`). So `alpha` < `alpha.1` < `beta`, `Beta` < `alpha`, and `rc.10` < `rc.2`.
  - Two versions are equal when all four fields are equal. `"1.2"` and `"1.2.0"` parse to the same
    version.
  - `Ord` and `PartialOrd` must follow these rules and agree with each other and with `==`. The
    tests compare versions with `==`, `<`, `>`, `cmp` and `partial_cmp`, call `sort()`, `max()`
    and `min()`, and put versions in a `BTreeSet` and a `HashSet`.
- `latest_stable` returns the greatest of the versions that have no pre-release, or `None` if there
  is no such version (in particular, if `versions` is empty).
- Do not change the definitions of `Version` and `ParseVersionError`.

### Examples

```rust
"1.2".parse::<Version>()      // Ok(Version { major: 1, minor: 2, patch: 0, pre: None })
"3-rc.1".parse::<Version>()   // Ok(Version { major: 3, minor: 0, patch: 0, pre: Some("rc.1") })
"1.2.3.4".parse::<Version>()  // Err(ParseVersionError)
```

Below, `1.9` stands for `"1.9".parse::<Version>().unwrap()`, and so on.

- `1.2` is displayed as `1.2.0`, and `1.0.0-beta` as `1.0.0-beta`.
- `1.9` < `1.10`, and `1.0.0-alpha` < `1.0.0-alpha.1` < `1.0.0-beta` < `1.0.0` < `1.0.1`.
- A `BTreeSet` of `1.2`, `1.2.0` and `1.2.0-rc1` has 2 elements.
- `latest_stable(&[1.0.0, 2.0.0-rc1, 1.4])` is `Some(&1.4.0)`.
