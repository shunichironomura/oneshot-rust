There are three errors, and the compiler reports all of them at once.

- **`[Vec::new(); 26]`**: a repeat expression `[value; N]` evaluates `value` once and copies the
  result into every element, so the element type must be `Copy`. A `Vec` owns a heap buffer, and 26
  bitwise copies of it would all point to the same buffer. That is why `Vec<String>` is not `Copy`.
  The exception is a constant: if `value` is a `const` block or the name of a constant, it is
  evaluated `N` times instead of being copied. `Vec::new` is a `const fn`, so `Vec::new()` can be
  wrapped in an inline `const` block: `[const { Vec::new() }; 26]` gives 26 separate empty `Vec`s.
- **`first.to_ascii_lowercase() - 'a'`**: `char` has no arithmetic at all, so convert to an integer
  first. `first` is an ASCII letter at this point, so `as u8` loses nothing, and `b'a'` is the byte
  `97`: `(first.to_ascii_lowercase() as u8 - b'a') as usize`.
- **`'a' + i`**: the same problem the other way round, and the way back to `char` is narrow: only a
  `u8` can be cast to `char`. `('a' as usize + i) as char` is rejected, and `(b'a' + i) as char`
  adds a `usize` to a `u8`. Convert the index instead: `(b'a' + i as u8) as char`.

Keep the code around the arithmetic as it is. Without `to_ascii_lowercase`, `b'A' - b'a'` underflows,
which panics in debug builds. With `is_alphabetic` instead of `is_ascii_alphabetic`, non-ASCII
letters pass the check, and `as u8` keeps only the low byte of their code point: `ñ` (U+00F1) gives
an index far past 25, and `š` (U+0161) even lands in bucket `a`.

### Alternatives

`std::array::from_fn` builds an array by calling a closure once for each index (the argument is the
index, unused here). It is the most general way, because the closure may compute anything, even a
value that cannot be a constant, such as `Vec::with_capacity(8)`.

```rust
let mut buckets: [Vec<String>; 26] = std::array::from_fn(|_| Vec::new());
```

Two more ways work here. A named constant can be repeated just like a `const` block:
`const EMPTY: Vec<String> = Vec::new();` and then `[EMPTY; 26]`. `Default::default()` works too:
an array implements `Default` when its element type does, but only up to length 32, so it would stop
compiling with 40 buckets.

A range of `char`s is an iterator, so `letters_used` can pair each letter with its bucket without
any arithmetic.

```rust
pub fn letters_used(buckets: &[Vec<String>; 26]) -> String {
    ('a'..='z')
        .zip(buckets)
        .filter(|(_, bucket)| !bucket.is_empty())
        .map(|(letter, _)| letter)
        .collect()
}
```

For the index, `first.to_ascii_lowercase() as usize - 'a' as usize` works too. A `char` can be
cast to any integer type; only the cast *to* `char` is limited to `u8`.
