`args` can be any iterator, so it has no `len()` and cannot be indexed: you take the items out one
at a time with `next()`. `next` takes `&mut self`, so the parameter has to be declared `mut args`.
That `mut` belongs to the local binding, not to the signature, and callers do not see it. Each
`next()` returns an `Option<String>` whose `String` is moved out of the iterator, so `query` and
`file_path` are taken over without a `clone`. `ok_or(...)?` turns a `None` into the error and
returns early.

The error messages are string literals, which have the type `&'static str`, so they can be returned
as they are. For the fourth item, a `String` cannot be matched against a literal pattern such as
`Some("--ignore-case")` (that is a type error). `as_deref()` turns the `Option<String>` into an
`Option<&str>` first; a guard such as `Some(arg) if arg == "--ignore-case"` works too. The Book's
version ignores extra arguments, so remember to check that no fifth item follows.

`str::lines` already handles `\r\n` and a final line ending; `split('\n')` would keep the `\r` and
produce an extra empty line after a final line ending.

`search` returns the `line`s that `contents.lines()` yields, which are slices of `contents`. The `'a`
in the signature ties the results to `contents` only, which is why they stay valid after `query` is
dropped.

### Alternatives

The style of the Book, with `match` expressions and a `for` loop.

```rust
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = match args.next() {
            None => false,
            Some(arg) if arg == "--ignore-case" => true,
            Some(_) => return Err("Unexpected argument"),
        };

        match args.next() {
            None => Ok(Config { query, file_path, ignore_case }),
            Some(_) => Err("Unexpected argument"),
        }
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
```

You can also collect the arguments into a `Vec` and match on the shape of the slice. Every case is
visible at a glance, but the strings in the slice are borrowed, so they have to be cloned, and all
the arguments are read even when there are too many.

```rust
pub fn build(args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    let args: Vec<String> = args.collect();
    let (query, file_path, ignore_case) = match args.as_slice() {
        [] | [_] => return Err("Didn't get a query string"),
        [_, _] => return Err("Didn't get a file path"),
        [_, query, file_path] => (query, file_path, false),
        [_, query, file_path, flag] if flag == "--ignore-case" => (query, file_path, true),
        _ => return Err("Unexpected argument"),
    };
    Ok(Config { query: query.clone(), file_path: file_path.clone(), ignore_case })
}
```
