The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

It is a string interner shared by the whole program. It gives every distinct string a number (its
id), so that the rest of the program can store and compare small numbers instead of strings and turn
them back into text when needed.

```rust
pub fn intern(s: &str) -> u32;
pub fn resolve(id: u32) -> Option<String>;
pub fn interned_count() -> usize;
pub fn intern_calls() -> usize;
```

- `intern(s)` returns the id of `s`. A string that is interned for the first time gets the next
  unused id: `0`, `1`, `2`, ... in the order in which strings are first interned. Interning an equal
  string again returns the same id.
- Strings are compared exactly: case matters and nothing is trimmed. Any string can be interned,
  including the empty string.
- `resolve(id)` returns the string with the id `id`, or `None` if that id has not been handed out.
- `interned_count()` returns the number of distinct strings interned so far. `intern_calls()`
  returns the total number of calls to `intern` so far, including calls with a string that was
  already interned. `resolve` and `interned_count` do not change either count.
- There is one table for the whole program, kept until the program ends. The functions may be
  called from several threads at the same time, and all threads share the table: an id returned on
  one thread resolves to the same string on every thread, a string gets exactly one id even when
  several threads intern it at once, and the ids handed out are always exactly
  `0..interned_count()`.

Do not change the four public signatures. Everything else (the private statics, the imports and the
function bodies) may be changed.

### Example

In a program that has not interned anything yet:

```rust
intern("apple");  // 0
intern("pear");   // 1
intern("apple");  // 0 (already interned)
resolve(1);       // Some("pear")
resolve(2);       // None
interned_count(); // 2
intern_calls();   // 3
```
