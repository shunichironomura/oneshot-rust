The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

Each line of a small data file holds a pair `a,b`: an integer `a` and a decimal number `b`.
`parse_pair` reads one pair, and `sum_pairs` adds up all the pairs of a file. Errors are reported
with the `AppError` type below (it is already in the editor, together with its `Display` and
`Error` impls).

```rust
#[derive(Debug, PartialEq)]
pub enum AppError {
    MissingComma,
    Parse(String),
}

impl fmt::Display for AppError { /* "missing comma" / "parse error: <msg>" */ }
impl Error for AppError {}

pub fn parse_pair(s: &str) -> Result<(i32, f64), AppError>
pub fn sum_pairs(text: &str) -> Result<f64, Box<dyn Error>>
```

- `parse_pair` splits `s` at its **first** `,` and trims both parts (as `str::trim` does). Then it
  parses `a` as an `i32` and `b` as an `f64` with `str::parse`.
- If `s` contains no `,`, the result is `Err(AppError::MissingComma)`.
- If a part does not parse, the result is `Err(AppError::Parse(msg))`, where `msg` is the
  `to_string()` of the `ParseIntError` or `ParseFloatError` that `parse` returned. If both parts
  are invalid, the error is the one for `a`.
- `AppError` is displayed as `missing comma` and `parse error: <msg>`.
- `sum_pairs` goes through the lines of `text` (as `str::lines` splits them) in order and skips the
  lines that are empty or contain only whitespace. Every other line is read with `parse_pair`, and
  the result is the sum of `a as f64 + b` over all of them (`0.0` if there are none).
- At the first line that `parse_pair` rejects, `sum_pairs` stops and returns that `AppError`,
  boxed. The tests get it back with `downcast_ref::<AppError>()`.

Do not change `AppError`, its `Display` and `Error` impls, or the signatures of `parse_pair` and
`sum_pairs`. Anything else may be changed.

### Examples

```rust
parse_pair("3, 0.5")          // Ok((3, 0.5))
parse_pair("x,1")             // Err(AppError::Parse("invalid digit found in string".to_string()))
parse_pair("1,abc")           // Err(AppError::Parse("invalid float literal".to_string()))

sum_pairs("1,0.5\n\n2,1.5")   // Ok(5.0)
sum_pairs("1,1\n2")           // Err(..), an AppError::MissingComma displayed as "missing comma"
```
