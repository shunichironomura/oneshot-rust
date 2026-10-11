The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

A `TempFile` is a temporary file kept in memory: when it is dropped, it is deleted, unless it has
been persisted. Each file reports what happens to it in a shared log.

```rust
pub type Log = Rc<RefCell<Vec<String>>>;

impl TempFile {
    pub fn create(name: &str, log: &Log) -> TempFile;
    pub fn write(&mut self, bytes: &[u8]);
    pub fn rename(self, new_name: &str) -> TempFile;
    pub fn persist(self) -> (String, Vec<u8>);
}

impl Drop for TempFile { /* ... */ }
```

- `create` creates an empty file named `name` that writes to `log`, and appends `"create <name>"` to
  the log. A name can be any string, even the empty string.
- `write` appends `bytes` to the contents of the file. It logs nothing.
- `rename` appends `"rename <old> -> <new>"` to the log, where `<old>` is the current name and
  `<new>` is `new_name`, and returns the file with the name `new_name` and the same contents.
  Renaming deletes nothing: only the `rename` line is logged, and when the renamed file is dropped
  later, it is deleted under its new name.
- `persist` consumes the file and returns its name and contents. A persisted file is never deleted:
  nothing is appended to the log, now or later.
- Dropping a file that has not been persisted deletes it: it appends `"delete <name> (<n> bytes)"`
  to the log, where `<n>` is the length of the contents in bytes (`"(1 bytes)"` for one byte).
- A file that is gone (dropped or persisted) must not keep the log alive: once no file that writes
  to `log` is left, `Rc::strong_count(&log)` is `1` again. Memory leaks are detected by Miri and
  fail the submission.

Do not change `Log`, the public signatures, the fields of `TempFile` or its `Drop` impl. `unsafe`
code is allowed.

### Examples

```rust
let log: Log = Rc::new(RefCell::new(Vec::new()));
let mut a = TempFile::create("a", &log);
a.write(b"hi");
let (name, data) = a.persist(); // name == "a", data == b"hi"
// Rc::strong_count(&log) == 1
let b = TempFile::create("b", &log);
drop(b);
// log: ["create a", "create b", "delete b (0 bytes)"]
```

```rust
let mut f = TempFile::create("draft", &log); // appends "create draft"
f.write(b"abc");
let f = f.rename("final");                   // appends "rename draft -> final"
drop(f);                                     // appends "delete final (3 bytes)"
```
