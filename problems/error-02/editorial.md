`Box<dyn Error>` can hold any error type, and `?` boxes errors for you: for every type `E` that
implements `Error`, the standard library has `From<E> for Box<dyn Error>`. So `line.parse()?`
returns the `ParseIntError` itself, inside a box, and the caller can get it back with
`downcast_ref::<ParseIntError>()`.

Plain messages are the tricky part. `&str` and `String` do not implement `Error`, so
`Err(Box::new("overflow"))` does not compile, and neither does `Err("no numbers")`, which is a
`Result<_, &str>`. But there is also `From<&str>` (and `From<String>`) for `Box<dyn Error>`, which
wraps the text in a hidden error type. So `"no numbers".into()` works, and `?` uses the same
conversion: `checked_add(n).ok_or("overflow")?` turns `None` into a boxed `"overflow"` error.

`checked_add` returns `None` instead of overflowing; a plain `sum += n` panics on overflow in debug
builds. Checking at every addition, in the same loop that parses, takes care of two rules at once:
the *running* sum is checked, and the first problem wins (parsing every line first would report a
later bad line before an earlier overflow). `seen` is needed because a sum of `0` does not mean
there were no numbers.

The tempting `line.parse::<i64>().map_err(|e| e.to_string())?` compiles too, through
`From<String>`, but the box then holds only a message. The message looks right, yet
`downcast_ref::<ParseIntError>()` returns `None`.

### Alternatives

Define your own error type for the messages. It needs `Debug`, `Display` and an (empty)
`impl Error`; then `?` and `Box::new` turn it into a `Box<dyn Error>`.

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct Problem(&'static str);

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl Error for Problem {}

pub fn total(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut sum: i64 = 0;
    let mut seen = false;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let n: i64 = line.parse()?;
        sum = sum.checked_add(n).ok_or(Problem("overflow"))?;
        seen = true;
    }
    if !seen {
        return Err(Box::new(Problem("no numbers")));
    }
    Ok(sum)
}
```

An `Option<i64>` can replace the `seen` flag: `None` means "no number yet", and `ok_or_else` turns
it into the `no numbers` error at the end.

```rust
use std::error::Error;

pub fn total(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut sum: Option<i64> = None;
    for line in input.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let n: i64 = line.parse()?;
        sum = Some(match sum {
            None => n,
            Some(s) => s.checked_add(n).ok_or("overflow")?,
        });
    }
    sum.ok_or_else(|| "no numbers".into())
}
```
