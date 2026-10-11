The solution makes each `Subscription` the owner of its listener and gives the subject only `Weak`
handles. Unsubscribing is then nothing more than dropping an `Rc`: `Subscription` needs no `Drop`
impl and no way back to its subject, the listener is freed as soon as its last `Rc` goes, and a
listener that captures an `Rc<Subject>` creates no cycle, because the subject owns nothing.

Every closure has its own unnamed type, so the list stores them as trait objects. In
`Listener { callback: Rc::new(f) }`, the field's type `Rc<dyn Fn(&str)>` makes the compiler coerce
the `Rc<F>` to an `Rc<dyn Fn(&str)>` (an unsized coercion), and `callback(msg)` calls the closure
through the `Rc`. `Rc<dyn Fn(&str)>` is short for `Rc<dyn Fn(&str) + 'static>`, which is why
`subscribe` requires `'static`: the list must not hold anything that borrows from the caller.

**Do not call listeners while the list is borrowed.** The methods take `&self`, so the list lives in
a `RefCell`. A loop like this one compiles:

```rust
for weak in self.listeners.borrow().iter() {
    if let Some(listener) = weak.upgrade() {
        (listener.callback)(msg);
    }
}
```

But the `Ref` returned by `borrow()` stays alive for the whole loop, and as soon as a listener calls
`subscribe`, its `borrow_mut()` panics with "RefCell already borrowed". `RefCell` checks the
borrowing rules at run time, and a callback that calls back into its caller breaks them. So `notify`
first copies the list (a `Vec` of `Weak`s, cheap to clone) and releases the borrow before calling
anyone. The copy also makes sure that listeners subscribed during a `notify` are not called by it.
Looping over indices and borrowing again for each one avoids the panic, but it calls the new
listeners too, and the indices shift when `subscribe` removes dead entries. Keeping the borrow and
having `subscribe` queue new listeners until `notify` returns also avoids it, but a listener
subscribed during a `notify` is subscribed at once: `listener_count` must count it, and a `notify`
started after it, even from inside a listener, must call it.

**Upgrade each `Weak` only when its turn comes.** A listener that was unsubscribed earlier in the
same `notify` then fails to upgrade and is skipped. Upgrading the whole copy at the start would keep
every listener alive until the end and call the unsubscribed ones anyway.

**Why `Listener` wraps the callback.** The obvious design stores `Weak<dyn Fn(&str)>` in the subject
and lets the `Subscription` own the `Rc<dyn Fn(&str)>`. It works until a listener unsubscribes
itself. While a listener runs, `notify` holds the `Rc` that `upgrade` returned, and it has to: the
closure must not be freed while it runs, and a `Weak` cannot be called anyway. So after the
`Subscription` is gone, the strong count is still 1. `strong_count() > 0` still counts the listener
(the second example would print `3 left`), and a `notify` started from inside the listener upgrades
it again and calls it again. A strong count counts every `Rc`, not only the one in the
`Subscription`.

The solution separates "subscribed" from "alive". The `Subscription` holds the only `Rc<Listener>`,
and the `Listener` holds the `Rc<dyn Fn(&str)>`. `notify` upgrades the `Weak<Listener>`, clones the
callback and drops the `Rc<Listener>` before the call. While a listener runs, only its callback is
kept alive, and the `Weak<Listener>` stops upgrading the moment the `Subscription` is dropped.

`subscribe` removes the dead `Weak`s with `retain`, so that the list does not keep growing, and
`listener_count` counts the ones that can still be upgraded. `#[must_use]` on `Subscription` makes
the compiler warn about a call like `subject.subscribe(f);`, which would unsubscribe the listener
right away.

### Alternatives

The subject can own the listeners instead, and each `Subscription` removes its entry in `drop`. The
`Subscription` then needs a way back to the list: here a `Weak` to an `Rc<RefCell<...>>` inside the
subject, which no longer upgrades once the subject is gone. Two details matter:

- `drop` must not drop the removed listener while the list is still borrowed. A listener may own
  other `Subscription`s, and their `drop` borrows the list again. A plain
  `listeners.borrow_mut().retain(...)` drops the listener in place and panics with "RefCell already
  borrowed". The code below removes the entry, ends the borrow, and only then drops it.
- `notify` copies only the ids and looks each listener up again right before its turn.

```rust
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

type Listeners = RefCell<Vec<(u64, Rc<dyn Fn(&str)>)>>;

pub struct Subject {
    listeners: Rc<Listeners>,
    next_id: Cell<u64>,
}

pub struct Subscription {
    listeners: Weak<Listeners>,
    id: u64,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        // The subject may already be gone.
        let Some(listeners) = self.listeners.upgrade() else { return };
        let removed = {
            let mut listeners = listeners.borrow_mut();
            let index = listeners.iter().position(|(id, _)| *id == self.id);
            index.map(|i| listeners.remove(i))
        };
        // Drop the listener only now that the list is no longer borrowed: it may own other
        // subscriptions, whose `drop` borrows the list again.
        drop(removed);
    }
}

impl Subject {
    pub fn new() -> Subject {
        Subject { listeners: Rc::new(RefCell::new(Vec::new())), next_id: Cell::new(0) }
    }

    pub fn subscribe(&self, f: impl Fn(&str) + 'static) -> Subscription {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        self.listeners.borrow_mut().push((id, Rc::new(f)));
        Subscription { listeners: Rc::downgrade(&self.listeners), id }
    }

    pub fn notify(&self, msg: &str) -> usize {
        let ids: Vec<u64> = self.listeners.borrow().iter().map(|(id, _)| *id).collect();
        let mut called = 0;
        for id in ids {
            // Look the listener up again: it may have been unsubscribed in the meantime.
            let listener = self
                .listeners
                .borrow()
                .iter()
                .find(|(i, _)| *i == id)
                .map(|(_, f)| Rc::clone(f));
            if let Some(f) = listener {
                f(msg);
                called += 1;
            }
        }
        called
    }

    pub fn listener_count(&self) -> usize {
        self.listeners.borrow().len()
    }
}
```

Or keep `Weak<dyn Fn(&str)>` in the subject, and tell whether the `Subscription` is alive by a
separate token: an `Rc<()>` that only the `Subscription` holds.

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone)]
struct Entry {
    /// Only the `Subscription` holds this `Rc`: it can be upgraded while the listener is
    /// subscribed, even while `notify` holds the callback.
    subscribed: Weak<()>,
    callback: Weak<dyn Fn(&str)>,
}

pub struct Subject {
    entries: RefCell<Vec<Entry>>,
}

pub struct Subscription {
    _subscribed: Rc<()>,
    _callback: Rc<dyn Fn(&str)>,
}

impl Subject {
    pub fn new() -> Subject {
        Subject { entries: RefCell::new(Vec::new()) }
    }

    pub fn subscribe(&self, f: impl Fn(&str) + 'static) -> Subscription {
        let subscribed = Rc::new(());
        let callback: Rc<dyn Fn(&str)> = Rc::new(f);
        let mut entries = self.entries.borrow_mut();
        entries.retain(|e| e.subscribed.strong_count() > 0);
        entries.push(Entry {
            subscribed: Rc::downgrade(&subscribed),
            callback: Rc::downgrade(&callback),
        });
        Subscription { _subscribed: subscribed, _callback: callback }
    }

    pub fn notify(&self, msg: &str) -> usize {
        let snapshot = self.entries.borrow().clone();
        let mut called = 0;
        for entry in snapshot {
            if entry.subscribed.strong_count() == 0 {
                continue;
            }
            if let Some(callback) = entry.callback.upgrade() {
                callback(msg);
                called += 1;
            }
        }
        called
    }

    pub fn listener_count(&self) -> usize {
        self.entries.borrow().iter().filter(|e| e.subscribed.strong_count() > 0).count()
    }
}
```
