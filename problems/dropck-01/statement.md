The code below does not compile. Fix it.

A `Tracer` appends a line to a shared sink when it is created and another one when it is dropped.
`trace` uses tracers to record a run of named steps.

```rust
pub struct Tracer<'a> {
    sink: &'a RefCell<Vec<String>>,
    name: String,
}

impl<'a> Tracer<'a> {
    pub fn new(sink: &'a RefCell<Vec<String>>, name: &str) -> Tracer<'a>;
}

impl Drop for Tracer<'_> { /* ... */ }

pub fn trace(names: &[&str]) -> Vec<String>;
```

- `Tracer::new(sink, name)` appends `"begin <name>"` to `sink` and returns a tracer.
- Dropping a tracer appends `"end <name>"` to its sink.
- `trace(names)` creates one tracer per name, in the order of `names`, all writing to the same sink.
  Then it appends `"run"` to the sink, drops all the tracers **in the order they were created**, and
  returns every line the sink collected, oldest first.
- `names` may be empty, and the same name may appear more than once.

`Tracer`, `Tracer::new` and the `Drop` impl are correct: do not change them. The `"end"` lines must
come from dropping the tracers. Do not change the signature of `trace` either.

### Example

`trace(&["a", "b"])` returns `["begin a", "begin b", "run", "end a", "end b"]`.
