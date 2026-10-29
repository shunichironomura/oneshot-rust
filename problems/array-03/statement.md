The code below does not compile. Fix it.

Words have been sorted into 26 buckets, one for each letter of the English alphabet: bucket `0` is
for `a`, bucket `1` for `b`, and so on up to bucket `25` for `z`.

```rust
pub fn letters_used(buckets: &[Vec<String>; 26]) -> String
```

- `letters_used` returns the lowercase letters whose buckets are not empty, in alphabetical order.
- It only looks at whether each bucket is empty, not at the strings inside.
- If every bucket is empty, it returns `""`.

Do not change the signature.

### Examples

- If bucket `0` is `["Al"]`, bucket `1` is `["bob", "beth"]` and all the other buckets are empty,
  `letters_used` returns `"ab"`.
- If bucket `24` is `["Yak"]`, bucket `25` is `["zoo", "zebra"]` and all the other buckets are
  empty, it returns `"yz"`.
