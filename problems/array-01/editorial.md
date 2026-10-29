Arrays own their elements, just like a `Vec`. Indexing only names a place inside the array, so
`(a[i], b[i])` cannot move an `A` out of `a`: that would leave a hole in the array. Copying the
element instead would need `Copy` or `Clone`, which `A` and `B` may not have. Starting from an
"empty" result such as `[None; N]` and filling it in fails too: a repeat expression copies
its value `N` times, so the element type must be `Copy`.

The way out is to consume the array. `array.map(f)` takes the array by value, passes each element to
`f` **by value** in order, and collects the results into a new array of the same length. Arrays also
implement `IntoIterator` by value, so `for x in arr` and (since edition 2021) `arr.into_iter()` yield
owned elements.

`zip_arrays` combines both: `a.map` hands out the elements of `a`, and an iterator over `b` hands
out the matching element of `b` on each call. The `unwrap` never fails, because both arrays have
`N` elements, and for `N = 0` the closure is not called at all.

### Alternatives

`std::array::from_fn(|i| ...)` builds an array from the indices. Direct indexing cannot move
non-`Copy` values out of the original arrays. One option is to take elements from an iterator:

```rust
pub fn zip_arrays<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N] {
    let mut pairs = a.into_iter().zip(b);
    std::array::from_fn(|_| pairs.next().unwrap())
}
```

Alternatively, wrap the inputs in arrays of `Option` with `let mut a = a.map(Some)` and
`let mut b = b.map(Some)`. Then use
`std::array::from_fn(|i| (a[i].take().unwrap(), b[i].take().unwrap()))` to move each pair out,
leaving `None` at each index. Each index is visited once, so the `unwrap` calls succeed.

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

If you do want such an array to fill in, an inline `const` block is repeated as a constant instead of
being copied: `[const { None }; N]` compiles as an `[Option<(A, B)>; N]`, which you can fill and then
turn into the result with `.map(Option::unwrap)`.
