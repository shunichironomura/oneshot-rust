A program wants to know how often each type has been used. Implement four functions that keep one
counter **per type** for the whole program.

```rust
pub fn register<T: 'static>() -> usize;
pub fn count<T: 'static>() -> usize;
pub fn total() -> usize;
pub fn distinct() -> usize;
```

- `register::<T>()` adds 1 to the counter of the type `T` and returns the new value of that counter.
  The first `register::<T>()` of a type returns `1`.
- `count::<T>()` returns the counter of `T`, which is `0` for a type that has never been registered.
  It does not change anything: in particular, it does not register `T`.
- `total()` returns the number of calls to `register` so far, over all types (the sum of all
  counters).
- `distinct()` returns the number of different types that have been registered at least once.
- Every type has its own counter, and types are told apart exactly as Rust tells them apart: a type
  alias is the same type as the type it names, `Vec<u8>` and `Vec<u16>` are different types, and two
  structs declared in different places are different types even when they have the same name.
- The counters belong to the whole program and are kept until it ends; nothing is ever reset. The
  functions may be called from several threads at the same time, and all threads share the
  counters. No call is lost, and calls for the same type never return the same value: `n` calls to
  `register::<T>()` return exactly `1`, `2`, ..., `n` in some order.

The harness declares its own types inside its test cases (unit structs such as `struct Apple;`) and
also uses standard types such as `Vec<u8>`. Do not change the signatures.

### Examples

In a program that has not registered anything yet:

```rust
struct Apple;
struct Pear;
struct Plum;

register::<Apple>(); // 1
register::<Apple>(); // 2
register::<Pear>();  // 1
count::<Apple>();    // 2
count::<Plum>();     // 0
total();             // 3
distinct();          // 2
```

A type alias is not a new type. If `Vec<u8>` and `Vec<u16>` have not been registered yet:

```rust
type Bytes = Vec<u8>;

register::<Vec<u8>>();  // 1
register::<Bytes>();    // 2 (Bytes is Vec<u8>)
register::<Vec<u16>>(); // 1
```
