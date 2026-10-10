The body only needs `>` on elements, so make the function generic over `T: PartialOrd`. `Ord` is not
enough, because floating-point numbers (`f64`) implement only `PartialOrd` (`NaN` is not comparable
to anything).

The returned reference points into `items`, and since `items` is the only reference parameter,
lifetime elision handles it.

### Alternatives

You can write the bound in a `where` clause instead.

```rust
pub fn largest<T>(items: &[T]) -> &T
where
    T: PartialOrd,
```
