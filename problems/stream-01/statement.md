The standard library has `Future`, but no `Stream`, the asynchronous counterpart of `Iterator`.
Write a small stream library from scratch. The skeleton below is already in the editor: the `Stream`
trait and the signature of `next` are given, and `Iter` and `Zip` already have fields, which you may
change.

```rust
pub trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

pub trait StreamExt: Stream {
    fn next(&mut self) -> impl Future<Output = Option<Self::Item>> + '_
    where
        Self: Unpin,
    { /* to implement */ }
}

impl<S: Stream + ?Sized> StreamExt for S {}

pub struct Iter<I> { /* fields */ }
pub fn iter<I: IntoIterator>(items: I) -> Iter<I::IntoIter>;
impl<I: Iterator> Stream for Iter<I> { type Item = I::Item; /* poll_next */ }

pub struct Zip<A: Stream, B: Stream> { /* fields */ }
pub fn zip<A: Stream + Unpin, B: Stream + Unpin>(a: A, b: B) -> Zip<A, B>;
impl<A: Stream + Unpin, B: Stream + Unpin> Stream for Zip<A, B> {
    type Item = (A::Item, B::Item);
    /* poll_next */
}
```

- `poll_next` returns the next item, `Ready(None)` at the end, or `Pending` after making sure that
  the task will be woken through `cx`, and a stream is never polled again after `Ready(None)`.
- `s.next()` returns a future that resolves to the next item of `s`, or to `None` if `s` has ended.
  Each time the future is polled, it polls `s` once, with the same `cx`, and returns `Pending` if
  `s` did. `next` can be called on any stream that is `Unpin`, including trait objects such as
  `dyn Stream<Item = u32> + Unpin`.
- `iter(items)` returns a stream of the items of `items`, in order. It is never `Pending`.
- `zip(a, b)` returns a stream of pairs: the first item of `a` with the first item of `b`, then the
  second with the second, and so on. Each time it is polled, it does the following:
  1. If it does not hold an item of `a`, it polls `a` once. If `a` returns `Pending`, `zip` returns
     `Pending` without polling `b`. If `a` has ended, `zip` returns `Ready(None)`. Otherwise `zip`
     now holds the item of `a`.
  2. It polls `b` once. If `b` returns `Pending`, `zip` returns `Pending` and keeps holding the item
     of `a` for its next poll. If `b` has ended, `zip` returns `Ready(None)`. Otherwise it returns
     the pair of the two items, and it no longer holds an item.

  `zip` passes its `cx` on to `a` and `b`, and it does not wake anything itself.
- Keep the trait definitions, signatures and `impl` bounds as given (so `iter` accepts any iterator
  and `zip` any two `Unpin` streams); you may add types, `impl` blocks and functions of your own.

The test harness runs the futures with an executor of its own, which polls a future again only
after it has been woken. It also has streams of its own, and counts how many times each is polled.
For example, `slow(items)` returns `Pending` (after waking the task) before each item and
before the end: `Pending`, the first item, `Pending`, the second item, ..., `Pending`,
`Ready(None)`.

### Examples

- Polling `iter(vec![1, 2, 3])` until it ends gives `Ready(Some(1))`, `Ready(Some(2))`,
  `Ready(Some(3))` and `Ready(None)`.
- After `let mut s = iter(["x"]);`, `s.next().await` is `Some("x")`, and then `None`.
- Polling `zip(iter(1..=3), slow(vec!['a', 'b']))` until it ends gives `Pending`,
  `Ready(Some((1, 'a')))`, `Pending`, `Ready(Some((2, 'b')))`, `Pending` and `Ready(None)`. The
  `iter` stream is polled 3 times (its item `3` gets no partner), and `slow` 6 times.
