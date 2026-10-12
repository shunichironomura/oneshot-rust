Software versions such as `1.10.2` or `2.0.0-rc.1` are compared part by part, and comparing them
as strings gives wrong answers. Implement the ordering of the type `Version` below, and a function
that finds the newest stable version. The type is already in the editor, together with its parsing
(`FromStr`) and display (`Display`).

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
}

impl Ord for Version { /* to implement */ }
impl PartialOrd for Version { /* to implement */ }

pub fn latest_stable(versions: &[Version]) -> Option<&Version>
```

`pre` is the pre-release label, such as `rc.1` in `2.0.0-rc.1`, or `None` for a release. The given
parser reads `MAJOR[.MINOR[.PATCH]][-PRE]`, where a missing `MINOR` or `PATCH` is 0.

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
- Do not change the definition of `Version`.

### Examples

Below, `1.9` stands for `"1.9".parse::<Version>().unwrap()`, and so on.

- `1.9` < `1.10`, and `1.0.0-alpha` < `1.0.0-alpha.1` < `1.0.0-beta` < `1.0.0` < `1.0.1`.
- A `BTreeSet` of `1.2`, `1.2.0` and `1.2.0-rc1` has 2 elements.
- `latest_stable(&[1.0.0, 2.0.0-rc1, 1.4])` is `Some(&1.4.0)`.
