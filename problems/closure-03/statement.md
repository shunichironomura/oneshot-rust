The code below does not compile. Fix it.

`tally` goes through a list of events, keeps a log of them and counts the errors.

```rust
pub fn tally(events: &[&str], limit: usize) -> (usize, Vec<String>)
```

- `tally` processes `events` in order and appends each event to the log. An event that starts with
  `ERR` is an error. The check is case-sensitive and looks only at the very start: `"ERROR 7"` is an
  error, but `"err"`, `"Err"` and `" ERR"` are not.
- Right after an event is appended, if the number of errors so far (including that event) equals
  `limit`, `"abort"` is appended to the log and the remaining events are ignored. `limit` is at
  least 1.
- `tally` returns the number of errors and the log.

Do not change the signature. Anything inside the function body may be changed.

### Examples

```rust
tally(&["ok", "ERR a", "ERR b", "ok"], 2)
// (2, ["ok", "ERR a", "ERR b", "abort"])

tally(&["ok"], 1)
// (0, ["ok"])
```
