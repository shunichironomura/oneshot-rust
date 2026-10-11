All handles share one heap allocation, `Inner<T>`, which holds the count next to the value.
`new` boxes it and turns the box into a raw pointer with `Box::leak` (or `Box::into_raw`). Every
clone copies that pointer and adds 1 to the count. The handle that drops the count to 0 turns the
pointer back into a box with `Box::from_raw` and drops it, which drops the value and frees the
memory. `fetch_sub` returns the previous count, so exactly one handle sees `1`. Reading the count
and decrementing it in two separate steps would let two threads both think they are not last (a
leak) or both think they are (a double free). `Clone` cannot simply be derived either:
`#[derive(Clone)]` requires `T: Clone` and copies the pointer without touching the count.

**Variance comes from the field types.** `*mut X` is invariant in `X`, so with a `*mut Inner<T>`
field `MyArc<&'static str>` is no longer a subtype of `MyArc<&'a str>`, and `shorten` fails with
"the struct `MyArc<T>` is invariant over the parameter `T`". `NonNull<X>` and `*const X` are
covariant. Covariance is sound here because a shared handle only hands out `&T`: `get_mut` needs
`&mut MyArc<T>`, which is invariant itself.

**`Send` and `Sync` must be written by hand,** because raw pointers (and `NonNull`) are neither.
Both need `T: Send + Sync`. Handles on different threads give out `&T` at the same time, which
requires `T: Sync`. And whichever thread drops the last handle drops the `T`, so sending a handle is
like sending the value: `T: Send`. `Sync` needs the same bounds, because a `&MyArc<T>` on another
thread can be cloned into a handle there. With only `T: Send` on `Send`, a `MyArc<Cell<u8>>` could
be cloned into two threads that both call `set`: a data race in safe code.

**Memory orderings.** `fetch_add` and `fetch_sub` keep the count itself correct with any ordering;
the ordering is about the *other* memory accesses around them.

- `clone` can use `Relaxed`. The caller already holds a handle, so the value cannot disappear, and
  nothing else has to become visible to another thread.
- `drop` decrements with `Release`: everything this thread did with the value happens before the
  decrement. The last handle then needs `Acquire` (here a fence) before freeing, so that all those
  uses on other threads happen before the free. With `Relaxed` (or `Release` without the fence),
  nothing orders another thread's last use of the value before the free, and Miri reports a data
  race between the two. Creating a reference (a "retag" in Miri's message) counts as a use.
- `get_mut` must load the count with `Acquire` for the same reason: the other thread read the value
  before dropping its handle, and those reads must happen before our writes through the `&mut T`.
- `strong_count` is only a snapshot, so `Relaxed` is fine.

The Nomicon's version of `Arc` also has a `PhantomData<ArcInner<T>>` field, "for the drop checker".
Since `MyArc` has its own `Drop` impl, the compiler already assumes that dropping a `MyArc<T>` may
use a `T`, so that field changes nothing for the drop checker here. The standard library needs it
because its `Drop` impl uses the unstable `#[may_dangle]` attribute, which removes that assumption.

### Alternatives

A single `AcqRel` decrement replaces the `Release` decrement and the `Acquire` fence. It is
slightly more expensive, because every decrement acquires, not only the last one.

```rust
impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        if self.inner().count.fetch_sub(1, Ordering::AcqRel) == 1 {
            // SAFETY: this was the last handle, so nothing else can reach the allocation.
            drop(unsafe { Box::from_raw(self.ptr.as_ptr()) });
        }
    }
}
```

A `*const Inner<T>` field is covariant too. The pointer comes from `Box::into_raw`, so writing
through it after a cast to `*mut` is allowed.

```rust
use std::ops::Deref;
use std::sync::atomic::{self, AtomicUsize, Ordering};

struct Inner<T> {
    count: AtomicUsize,
    value: T,
}

pub struct MyArc<T> {
    ptr: *const Inner<T>,
}

unsafe impl<T: Send + Sync> Send for MyArc<T> {}
unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

impl<T> MyArc<T> {
    pub fn new(value: T) -> Self {
        MyArc { ptr: Box::into_raw(Box::new(Inner { count: AtomicUsize::new(1), value })) }
    }

    fn inner(&self) -> &Inner<T> {
        unsafe { &*self.ptr }
    }

    pub fn strong_count(this: &Self) -> usize {
        this.inner().count.load(Ordering::Relaxed)
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        if this.inner().count.load(Ordering::Acquire) == 1 {
            Some(unsafe { &mut (*this.ptr.cast_mut()).value })
        } else {
            None
        }
    }

    pub fn ptr_eq(a: &Self, b: &Self) -> bool {
        a.ptr == b.ptr
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        self.inner().count.fetch_add(1, Ordering::Relaxed);
        MyArc { ptr: self.ptr }
    }
}

impl<T> Deref for MyArc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner().value
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        if self.inner().count.fetch_sub(1, Ordering::Release) == 1 {
            atomic::fence(Ordering::Acquire);
            drop(unsafe { Box::from_raw(self.ptr.cast_mut()) });
        }
    }
}
```

The value and the count can also live in two separate boxes. Then `ptr_eq` must compare the
pointers to the counts: a `Box` of a zero-sized value allocates nothing, so two `MyArc::new(())`
would have equal value pointers.
