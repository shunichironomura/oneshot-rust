Arrays own their elements, just like a `Vec`. Indexing only names a place inside the array, so
`(a[i], b[i])` cannot move an `A` out of `a`: that would leave a hole in the array. Copying the
element instead would need `Copy` or `Clone`, which `A` and `B` may not have. Starting from an
"empty" result such as `[String::new(); N]` and filling it in fails too: a repeat expression copies
its value `N` times, so the element type must be `Copy`.

The way out is to consume the array. `array.map(f)` takes the array by value, passes each element to
`f` **by value** in order, and collects the results into a new array of the same length. That is all
`shout` needs. Arrays also implement `IntoIterator` by value, so `for x in arr` and (since edition
2021) `arr.into_iter()` yield owned elements.

`zip_arrays` combines both: `a.map` hands out the elements of `a`, and an iterator over `b` hands out
the matching element of `b` on each call. The `unwrap` never fails, because both arrays have `N`
elements, and for `N = 0` the closure is not called at all.

`transpose` works with `i32`, which is `Copy`, so `[[0; R]; C]` is fine here. Note the swapped
lengths in the return type. Take the sizes from `R` and `C` (or let the loops run over `m`) rather
than from `m[0].len()`: with `R = 0`, `m[0]` panics.

### Alternatives

`std::array::from_fn(|i| ...)` builds an array from the indices. It works for `transpose`, because
the numbers are copied out of `m`.

```rust
pub fn transpose<const R: usize, const C: usize>(m: [[i32; C]; R]) -> [[i32; R]; C] {
    std::array::from_fn(|c| std::array::from_fn(|r| m[r][c]))
}
```

For `zip_arrays`, `from_fn` works only if the closure takes the elements from an iterator instead of
indexing.

```rust
pub fn zip_arrays<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N] {
    let mut pairs = a.into_iter().zip(b);
    std::array::from_fn(|_| pairs.next().unwrap())
}
```

You can also collect into a `Vec` and convert it back to an array. `.try_into().unwrap()` does not
compile here: the error of the conversion is the `Vec` itself, and `unwrap` needs it to implement
`Debug`, which `A` and `B` do not promise. Match on the result instead.

```rust
pub fn zip_arrays<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N] {
    let pairs: Vec<(A, B)> = a.into_iter().zip(b).collect();
    match pairs.try_into() {
        Ok(array) => array,
        Err(_) => unreachable!("both arrays have N elements"),
    }
}
```

`shout` can also change the strings in place, because the function owns the array.

```rust
pub fn shout<const N: usize>(mut names: [String; N]) -> [String; N] {
    for name in &mut names {
        *name = name.to_uppercase();
        name.push('!');
    }
    names
}
```

If you do want an array of `String`s to fill in, an inline `const` block is repeated as a constant
instead of being copied: `[const { String::new() }; N]` compiles.
