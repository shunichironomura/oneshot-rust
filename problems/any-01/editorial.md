`is`, `downcast_ref`, `downcast_mut` and `downcast` are inherent methods of `dyn Any` and
`Box<dyn Any>`, not methods of the `Any` trait, so they cannot be called on a `dyn Component`. The
component has to become a `dyn Any` first. Since Rust 1.86 this is a plain coercion called *trait
upcasting*: `Any` is a supertrait of `Component`, so `&dyn Component` coerces to `&dyn Any`,
`&mut dyn Component` to `&mut dyn Any`, and `Box<dyn Component>` to `Box<dyn Any>`. The concrete
type is kept, so the downcast checks the real type of the component. `Component: Any` also makes
every component type `'static`, which is what `downcast_ref::<C>()` requires of `C`.

The trap is *what* gets upcast. The items are `Box<dyn Component>`, and a `Box<dyn Component>` is a
`'static` type itself, so it implements `Any` too. When `item` is a `&Box<dyn Component>` (as in
`for item in &self.items`), `item as &dyn Any` or `let any: &dyn Any = item;` does not look inside
the box: it turns the `Box` itself into a `dyn Any`, whose type is `Box<dyn Component>`. This
compiles without a warning, and every downcast fails, so `get` returns `None` and `count` returns
`0`. `item.type_id()` has the same problem: method lookup finds
`<Box<dyn Component> as Any>::type_id` before it reaches the component. (`item.label()` is fine,
because the `Box` does not implement `Component`.) `&**item` is the component: the first `*` follows
the reference to the `Box`, the second goes into the `Box`.

`take_all` has to move the components out. `Box<dyn Any>::downcast::<C>()` returns
`Result<Box<C>, Box<dyn Any>>`, and `*boxed` moves the value out of the `Box<C>`. But a failed
downcast only gives back a `Box<dyn Any>`, which cannot become a `Box<dyn Component>` again: there
is no downcasting to a trait. So check the type with `is::<C>()` through a reference first, and
upcast only the boxes that match; the `unwrap` then cannot fail. Taking all the items out with
`std::mem::take` and splitting them into two `Vec`s keeps both groups in order.

### Alternatives

`Vec::extract_if` removes the matching items in order and leaves the others in place.

```rust
pub fn take_all<C: Component>(&mut self) -> Vec<C> {
    self.items
        .extract_if(.., |item| (&**item as &dyn Any).is::<C>())
        .map(|item| *(item as Box<dyn Any>).downcast::<C>().unwrap())
        .collect()
}
```

Before Rust 1.86 there was no upcasting, and the usual workaround was a helper trait with a blanket
impl. The harness's `impl Component for ...` blocks still compile, because every type gets `AsAny`
from the blanket impl. The same trap applies: `Box<dyn Component>` gets the blanket impl too, so
`item.as_any()` on a `&Box<dyn Component>` returns the box. Call it on `**item`.

```rust
use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

pub trait Component: AsAny {
    fn label(&self) -> String;
}

pub struct World {
    items: Vec<Box<dyn Component>>,
}

impl World {
    pub fn new() -> Self {
        World { items: Vec::new() }
    }

    pub fn add<C: Component>(&mut self, c: C) {
        self.items.push(Box::new(c));
    }

    pub fn get<C: Component>(&self) -> Option<&C> {
        self.items.iter().find_map(|item| (**item).as_any().downcast_ref::<C>())
    }

    pub fn get_mut<C: Component>(&mut self) -> Option<&mut C> {
        self.items
            .iter_mut()
            .find_map(|item| (**item).as_any_mut().downcast_mut::<C>())
    }

    pub fn count<C: Component>(&self) -> usize {
        self.items.iter().filter(|item| (***item).as_any().is::<C>()).count()
    }

    pub fn take_all<C: Component>(&mut self) -> Vec<C> {
        let mut taken = Vec::new();
        let mut kept = Vec::new();
        for item in std::mem::take(&mut self.items) {
            if (*item).as_any().is::<C>() {
                taken.push(*item.into_any().downcast::<C>().unwrap());
            } else {
                kept.push(item);
            }
        }
        self.items = kept;
        taken
    }

    pub fn labels(&self) -> Vec<String> {
        self.items.iter().map(|item| item.label()).collect()
    }
}
```
