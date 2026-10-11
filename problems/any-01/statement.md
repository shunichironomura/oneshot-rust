A small game engine keeps the components of an entity in a `World`. A component can be a value of
any type that implements the trait `Component`. The `World` stores all of them in one list as trait
objects and finds them again by their concrete type. The trait and the struct are already in the
editor. Implement the methods of `World`.

```rust
use std::any::Any;

pub trait Component: Any {
    fn label(&self) -> String;
}

pub struct World {
    items: Vec<Box<dyn Component>>,
}

impl World {
    pub fn new() -> Self;
    pub fn add<C: Component>(&mut self, c: C);
    pub fn get<C: Component>(&self) -> Option<&C>;
    pub fn get_mut<C: Component>(&mut self) -> Option<&mut C>;
    pub fn count<C: Component>(&self) -> usize;
    pub fn take_all<C: Component>(&mut self) -> Vec<C>;
    pub fn labels(&self) -> Vec<String>;
}
```

- `new` returns an empty world. `add` appends `c` at the end, so the items are kept in the order in
  which they were added.
- An item *is a `C`* when its concrete type is exactly `C`. Items of different types are never
  confused, even when their labels are equal.
- `get` returns the first item that is a `C`, or `None` if there is none. `get_mut` does the same
  with a mutable reference, and changes made through it change the item stored in the world.
- `count` returns how many items are a `C`.
- `take_all` removes every item that is a `C` from the world and returns them by value, in their
  order in the world. The other items stay in the world in their original order. If no item is a
  `C`, it returns an empty `Vec` and the world does not change.
- `labels` returns the `label()` of every item, in order.

The harness defines its own component types: `Pos(i32)` and `Vel(i32)` with the labels `"pos"` and
`"vel"`, a zero-sized `Tag` with the label `"tag"`, and `Name(String)`, whose label is its string.

### Examples

```rust
let mut w = World::new();
w.add(Pos(1));
w.add(Vel(2));
w.add(Pos(3));
w.count::<Pos>()    // 2
w.get::<Vel>()      // Some(&Vel(2))
w.take_all::<Pos>() // [Pos(1), Pos(3)]
w.labels()          // ["vel"]
```

```rust
let mut w = World::new();
w.add(Vel(5));
w.add(Tag);
w.add(Vel(7));
w.get_mut::<Vel>().unwrap().0 += 1;
w.get::<Vel>()      // Some(&Vel(6))
w.get::<Pos>()      // None
w.labels()          // ["vel", "tag", "vel"]
```
