`Deref` with `Target = [T]` is what lets `SortedVec<T>` be read like a slice. In `deref`,
`&self.items` is a `&Vec<T>`, and deref coercion turns it into the `&[T]` that the signature asks
for. Callers get the rest for free: a method call such as `sv.len()` or `sv.windows(2)` auto-derefs
`SortedVec<T>` to `[T]` when `SortedVec` has no method of that name, indexing like `sv[0]` goes
through the same auto-deref, and `&sv` coerces to `&[T]` when it is passed to a function that takes
a slice. Since there is no `DerefMut`, all of this is read-only: `sv.swap(0, 1)`, `sv.sort_by(...)`
or `sv[0] = x` need a `&mut [T]` and do not compile, so only `insert` and `remove` can change the
contents and the order cannot break.

Methods of `SortedVec` itself come before the slice methods. Inside the `impl`,
`self.insert(index, value)` calls `SortedVec::insert` (which takes one argument), not `Vec::insert`,
so write `self.items.insert(...)` and `self.items.remove(...)`.

With equal elements, `binary_search` returns *some* matching index, and the documentation does not
say which one. (Today it happens to return the last one, so a `remove` built on it takes the wrong
element.) `partition_point(pred)` instead returns the first index where `pred` is false:

- `partition_point(|x| x <= &value)` is the first element greater than `value`, just after all the
  equal ones: where `insert` puts `value`.
- `partition_point(|x| x < value)` is the first element that is not less than `value`: the leftmost
  equal one, if there is one. `self.items.get(index) == Some(value)` checks that, and also handles
  `index == len`.

In the closure of `insert`, `x` is a `&T` but `value` is a `T`, so compare `x` with `&value` (or
write `*x <= value`: comparison operators take their operands by reference, so nothing is moved).
`|x| x <= value` compares a `&T` with a `T` and does not compile, and `|&x| x <= value` would move a
`T` out of a reference, which only works for `Copy` types such as `i32`.

`sort` is stable, while `sort_unstable` may reorder equal elements (it does for longer inputs). To
sort the vector in place, the parameter of `from` must be declared `mut items`.

### Alternatives

Linear scans are simpler. `Vec::insert` and `Vec::remove` shift the following elements anyway, so
both methods stay O(n).

```rust
pub fn insert(&mut self, value: T) -> usize {
    let index = self
        .items
        .iter()
        .position(|x| x > &value)
        .unwrap_or(self.items.len());
    self.items.insert(index, value);
    index
}

pub fn remove(&mut self, value: &T) -> bool {
    match self.items.iter().position(|x| x == value) {
        Some(index) => {
            self.items.remove(index);
            true
        }
        None => false,
    }
}
```

A comparison that never answers `Equal` makes `binary_search_by` return `Err` with the boundary.
`then(Ordering::Less)` treats equal elements as smaller, which gives the insertion point (with
`use std::cmp::Ordering;`). `then(Ordering::Greater)` treats them as greater instead, which gives the
position of the leftmost equal element, if there is one.

```rust
pub fn insert(&mut self, value: T) -> usize {
    let index = self
        .items
        .binary_search_by(|x| x.cmp(&value).then(Ordering::Less))
        .unwrap_err();
    self.items.insert(index, value);
    index
}
```

`from` can also insert the elements one by one. Each one goes after the equal elements already
there, so equal elements keep their input order, at a cost of O(n²) instead of O(n log n).

```rust
fn from(items: Vec<T>) -> Self {
    let mut sorted = SortedVec::new();
    for item in items {
        sorted.insert(item);
    }
    sorted
}
```
