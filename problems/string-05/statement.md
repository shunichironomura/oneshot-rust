The code below does not compile. Fix it.

`initials` abbreviates a full name to its initials.

```rust
pub fn initials(full_name: &str) -> String
```

- It returns the first character of each word of `full_name`, each followed by a `.`, in order.
- Words are separated by one or more whitespace characters (spaces, tabs, newlines), and whitespace
  at the start or end is ignored.
- The characters are kept as they are, with no change of case, and a word may start with any Unicode
  character, not only an ASCII letter.
- If there is no word, the result is `""`.

Do not change the signature.

### Examples

- `initials("Grace Brewster Hopper")` is `"G.B.H."`.
- `initials("émile zola")` is `"é.z."`.
