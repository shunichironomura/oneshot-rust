The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

`tally` goes through a list of events and keeps a log of them. `make_greeter` builds a function that
greets people.

```rust
pub fn tally(events: &[&str], limit: usize) -> (usize, Vec<String>)
pub fn make_greeter(greeting: String) -> impl Fn(&str) -> String
```

- `tally` processes `events` in order and appends each event to the log. An event that starts with
  `ERR` is an error. The check is case-sensitive and looks only at the very start: `"ERROR 7"` is an
  error, but `"err"`, `"Err"` and `" ERR"` are not.
- Right after an event is appended, if the number of errors so far equals `limit`, `"abort"` is
  appended to the log and the remaining events are ignored. `limit` is at least 1.
- `tally` returns the number of errors and the log.
- `make_greeter(greeting)` returns a function that turns `name` into `"<greeting>, <name>!"`, where
  `<greeting>` is `greeting` with the whitespace at both ends removed (as `str::trim` does). `name`
  is used as is. The returned function can be called any number of times.

Do not change the signatures. Anything inside the function bodies may be changed.

### Examples

```rust
tally(&["ok", "ERR a", "ERR b", "ok"], 2)
// (2, ["ok", "ERR a", "ERR b", "abort"])

tally(&["ok"], 1)
// (0, ["ok"])

let hi = make_greeter(String::from("  Hi "));
hi("Ann")  // "Hi, Ann!"
```
