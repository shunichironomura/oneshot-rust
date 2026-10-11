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

pub async fn collect<S: Stream + Unpin>(s: S) -> Vec<S::Item>;
```

- `poll_next` returns `Ready(Some(item))` with the next item, `Ready(None)` when the stream has
  ended, or `Pending` when no item is available yet. A stream that returns `Pending` makes sure that
  the task will be woken (through the waker of `cx`). A stream is never polled again after it has
  returned `Ready(None)`.
- `s.next()` returns a future that resolves to the next item of `s`, or to `None` if `s` has ended.
  Each time the future is polled, it polls `s` once, with the same `cx`, and returns `Pending` if
  `s` did. `next` can be called on any stream that is `Unpin`, including trait objects such as
  `dyn Stream<Item = u32> + Unpin`.
- `iter(items)` returns a stream of the items of `items`, in order. It is never `Pending`: each poll
  returns the next item, or `Ready(None)` when there are no more.
- `zip(a, b)` returns a stream of pairs: the first item of `a` with the first item of `b`, then the
  second with the second, and so on. Each time it is polled, it does the following:
  1. If it does not hold an item of `a`, it polls `a` once. If `a` returns `Pending`, `zip` returns
     `Pending` without polling `b`. If `a` has ended, `zip` returns `Ready(None)`. Otherwise `zip`
     now holds the item of `a`.
  2. It polls `b` once. If `b` returns `Pending`, `zip` returns `Pending` and keeps holding the item
     of `a` for its next poll. If `b` has ended, `zip` returns `Ready(None)`. Otherwise it returns
     the pair of the two items, and it no longer holds an item.

  `zip` passes its `cx` on to `a` and `b`, and it does not wake anything itself.
- `collect(s)` resolves to all the items of `s`, in order. Each time its future is polled, it polls
  `s` until `s` returns `Pending` (then the future returns `Pending`) or `Ready(None)` (then the
  future resolves).
- Do not change the trait definitions, the signatures, or the bounds of the given `impl` blocks:
  `iter` must accept any iterator, and `zip` any two streams that are `Unpin`, whatever their item
  types are. You may add types, `impl` blocks and functions of your own.

The test harness runs the futures with an executor of its own, which polls a future again only
after it has been woken. It also uses streams of its own, and it counts how many times each stream
is polled. For example, `slow(items)` returns `Pending` (after waking the task) before each item and
before the end: `Pending`, the first item, `Pending`, the second item, ..., `Pending`,
`Ready(None)`.

### Examples

- `collect(iter(vec![1, 2, 3]))` resolves to `[1, 2, 3]` on its first poll, and the `iter` stream
  is polled 4 times.
- After `let mut s = iter(["x"]);`, `s.next().await` is `Some("x")`, and then `None`.
- Polling `zip(iter(1..=3), slow(vec!['a', 'b']))` until it ends gives `Pending`,
  `Ready(Some((1, 'a')))`, `Pending`, `Ready(Some((2, 'b')))`, `Pending` and `Ready(None)`. The
  `iter` stream is polled 3 times (its item `3` gets no partner), and `slow` 6 times.
