The error is `first.to_ascii_lowercase() - 'a'`: `char` has no arithmetic at all, not even the
difference of two `char`s, and Rust never converts a `char` to an integer on its own. Convert to an
integer first. `first` is an ASCII letter at this point, so `as u8` loses nothing, and `b'a'` is the
byte `97`: `(first.to_ascii_lowercase() as u8 - b'a') as usize`.

`Default::default()` is fine as it is: an array implements `Default` when its element type does (up
to length 32), so it gives 26 separate empty `Vec`s.

Keep the code around the arithmetic as it is. Without `to_ascii_lowercase`, `b'A' - b'a'` underflows,
which panics in debug builds. With `is_alphabetic` instead of `is_ascii_alphabetic`, non-ASCII
letters pass the check, and `as u8` keeps only the low byte of their code point: `ñ` (U+00F1) gives
an index far past 25, and `š` (U+0161) even lands in bucket `a`.

### Alternatives

`first.to_ascii_lowercase() as usize - 'a' as usize` works too. A `char` can be cast to any integer
type; only the cast *to* `char` is limited to `u8`.
