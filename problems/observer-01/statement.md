A *subject* sends messages to the *listeners* subscribed to it. `subscribe` returns a
`Subscription`, and the listener stays subscribed exactly as long as that value is alive: dropping
it unsubscribes the listener. Listeners may use the subject while it is calling them, for example to
subscribe another listener or to drop their own `Subscription`. Implement `Subject` and
`Subscription`.

```rust
pub struct Subject { /* fields up to you */ }
pub struct Subscription { /* fields up to you */ }

impl Subject {
    pub fn new() -> Subject;
    pub fn subscribe(&self, f: impl Fn(&str) + 'static) -> Subscription;
    pub fn notify(&self, msg: &str) -> usize;
    pub fn listener_count(&self) -> usize;
}
```

- `new` creates a subject without listeners.
- `subscribe(f)` subscribes the listener `f` and returns its `Subscription`. The listener stays
  subscribed until that `Subscription` is dropped, however that happens: with `drop`, at the end of
  a scope, right away with `let _ = ...`, or because the value that owns it (another listener, for
  example) is dropped.
- `notify(msg)` calls each listener that is subscribed when it starts, passing `msg`, one at a time
  in the order in which they were subscribed, and returns how many listeners it called. A listener
  that is unsubscribed before its turn comes is not called.
- `listener_count` returns the number of subscribed listeners.
- Unsubscribing takes effect at once, even for a listener that is running at that moment (for
  example, one that drops its own `Subscription`): from then on, `listener_count` does not count it,
  and no `notify` calls it, including a `notify` started from inside its current call. The current
  call itself continues normally.
- While a listener is running, it may call `subscribe`, `notify` and `listener_count` on the same
  subject, and drop any `Subscription`. None of this may panic. A `notify` started from inside a
  listener follows the same rules: it calls the listeners subscribed at that moment, including
  running ones that are still subscribed. A listener subscribed during a `notify` is therefore not
  called by that `notify`, but by the ones started after it.
- The subject must not keep listeners alive. When a `Subscription` is dropped, its listener and
  everything the listener captured are freed right away. If this happens while a `notify` is
  running, the listener may instead stay alive until the outermost running `notify` returns.
- A listener may hold an `Rc` of its own subject. A `Subscription` may outlive its subject; dropping
  it then does nothing else. Nothing may leak.

The code above is already in the editor. Keep the signatures; the fields of both structs are up to
you.

### Examples

```rust
let subject = Subject::new();
let a = subject.subscribe(|msg| println!("a got {msg}"));
let _b = subject.subscribe(|msg| println!("b got {msg}"));
subject.listener_count(); // 2
subject.notify("hi");     // prints "a got hi", then "b got hi", and returns 2
drop(a);
subject.listener_count(); // 1
subject.notify("bye");    // prints "b got bye", and returns 1
```

A listener that unsubscribes itself the first time it is called:

```rust
let subject = Rc::new(Subject::new());
let _a = subject.subscribe(|msg| println!("a got {msg}"));

let slot: Rc<RefCell<Option<Subscription>>> = Rc::new(RefCell::new(None));
let (me, s) = (Rc::clone(&slot), Rc::clone(&subject));
let once = subject.subscribe(move |msg| {
    drop(me.borrow_mut().take()); // drops its own Subscription
    println!("once got {msg}, {} left", s.listener_count());
});
*slot.borrow_mut() = Some(once);

let _c = subject.subscribe(|msg| println!("c got {msg}"));
subject.notify("1"); // prints "a got 1", "once got 1, 2 left", "c got 1", and returns 3
subject.notify("2"); // prints "a got 2", "c got 2", and returns 2
```
