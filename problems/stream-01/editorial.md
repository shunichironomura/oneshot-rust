**`next`.** `std::future::poll_fn` turns a closure `FnMut(&mut Context<'_>) -> Poll<T>` into a
future, which is all `next` needs: each poll of the future polls the stream once. Two details make
it compile:

- `poll_next` takes `Pin<&mut Self>`, and a method call does not wrap a `&mut Self` in a `Pin` for
  you, so `self.poll_next(cx)` is an error. `Pin::new` makes one, but only when `Self: Unpin`, which
  is why `next` requires it.
- `Pin::new(self)` moves `self` (a `&mut Self`, which is not `Copy`) into the first call, so the
  closure could run only once, while `poll_fn` needs an `FnMut` (E0507). `Pin::new(&mut *self)`
  reborrows it for one call instead. The future keeps borrowing the stream, as the `+ '_` of the
  return type says.

**`Unpin` of `Iter` and `Zip`.** To modify the fields of `self: Pin<&mut Self>`, you need
`get_mut` (or `DerefMut`, with `mut self`), and both require `Self: Unpin`. `Unpin` is an auto
trait: `Iter<I>` is `Unpin` only if `I` is, and `Zip<A, B>` only if `A`, `B` *and* `A::Item` are,
since `Zip` stores an item of `a`. The impl blocks promise nothing about `I` or `A::Item`, so
`self.get_mut()` fails with "`I` cannot be unpinned" and "`<A as Stream>::Item` cannot be
unpinned". The compiler suggests adding `I: Unpin` and `<A as Stream>::Item: Unpin`, but that
changes the given bounds, and the harness uses an iterator that is not `Unpin`, as well as items
that are futures of `async` blocks, which are not `Unpin` either.

Instead, declare that pinning an `Iter` or a `Zip` does not pin what is inside:

```rust
impl<I> Unpin for Iter<I> {}
impl<A: Stream, B: Stream> Unpin for Zip<A, B> {}
```

`Unpin` is a safe trait, and these impls are correct because the code never makes a `Pin<&mut I>` or
a `Pin<&mut A::Item>` (there is no "structural pinning"): the iterator is used through a plain
`&mut I`, and the held item is only moved in and out. `a` and `b` *are* polled through a `Pin`, but
`Pin::new(&mut this.a)` requires `A: Unpin`, which the impl block guarantees. The streams of the
`futures` crate do the same. The impl must repeat the bounds of the struct:
`impl<A, B> Unpin for Zip<A, B> {}` is rejected, because `Zip<A, B>` requires `A: Stream` and
`B: Stream`.

**Holding the item in `zip`.** Once the item of `a` has been taken, it must survive a `Pending` from
`b`. The tempting `match ready!(Pin::new(&mut this.b).poll_next(cx)) { ... }` returns from
`poll_next` when `b` is `Pending` and drops the item of `a`, so the next poll pairs the *next* item
of `a` with the item of `b`. Put the item back into `held` before returning `Pending`, as the model
solution does. Do not poll `a` while you hold an item, and do not poll `b` before you have one: the
harness counts the polls.

### Alternatives

`next` can return a future written by hand instead of `poll_fn`. `Self` in a trait may be unsized
(the blanket impl keeps `?Sized`, so that `next` works on `dyn Stream`), so the struct must accept
unsized streams too: without `S: ?Sized`, `Next { stream: self }` does not compile ("the size for
values of type `Self` cannot be known"). `Next` only holds a reference, so it is `Unpin`, and
`self.stream` can be reached through `DerefMut`.

```rust
pub trait StreamExt: Stream {
    fn next(&mut self) -> impl Future<Output = Option<Self::Item>> + '_
    where
        Self: Unpin,
    {
        Next { stream: self }
    }
}

struct Next<'a, S: ?Sized> {
    stream: &'a mut S,
}

impl<S: Stream + Unpin + ?Sized> Future for Next<'_, S> {
    type Output = Option<S::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut *self.stream).poll_next(cx)
    }
}
```

Instead of the `impl Unpin` blocks, you can put what may not be `Unpin` in a `Box`. `Box<T>` is
`Unpin` for every `T`, because moving the box does not move its contents. This costs an allocation
per `Iter` and per held item.

```rust
pub struct Iter<I> {
    iter: Box<I>,
}

pub fn iter<I: IntoIterator>(items: I) -> Iter<I::IntoIter> {
    Iter { iter: Box::new(items.into_iter()) }
}

pub struct Zip<A: Stream, B: Stream> {
    a: A,
    b: B,
    held: Option<Box<A::Item>>,
}
```

In `Zip::poll_next`, the held item is then taken with `Some(item) => *item` and put back with
`this.held = Some(Box::new(a_item))`.
