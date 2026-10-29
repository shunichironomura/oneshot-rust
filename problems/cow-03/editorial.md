`Rc` only gives shared access, so `self.lines[i] = ...` does not compile. `Rc::make_mut` is
clone-on-write for `Rc`: if other `Rc`s point to the same value, it clones the value into a new
allocation owned by this `Rc` alone; otherwise it returns `&mut` to the value in place. `Rc::get_mut`
is the strict version that returns `None` when the value is shared.

`==` on two `Rc`s compares the values, so two separately created `Doc`s with equal lines would look
shared. `Rc::ptr_eq` compares the pointers instead.

### Alternatives

`set_line` can do by hand what `make_mut` does, with `Rc::strong_count` and `Rc::get_mut`. Be careful
to clone the `Vec`: `self.lines.clone()` would only clone the `Rc`.

```rust
pub fn set_line(&mut self, i: usize, text: &str) {
    if Rc::strong_count(&self.lines) > 1 {
        self.lines = Rc::new(Vec::clone(&self.lines));
    }
    Rc::get_mut(&mut self.lines).unwrap()[i] = text.to_string();
}
```
