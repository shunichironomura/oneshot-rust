A function that returns `impl Trait` still returns **one** concrete type. The body picks that type,
and callers only see "some type that implements the trait". It does not mean "any value that
implements the trait", so every branch has to produce the same type:

- In `select`, every closure has its own anonymous type, even when two closures look the same. So
  `Filter<Copied<Iter<i32>>, {closure#1}>` and `Filter<Copied<Iter<i32>>, {closure#2}>` are
  different types, and `Copied<Iter<i32>>` is a third one.
- In `describe`, `String` and `i32` both implement `Display`, but they are still two types.
- In `op_for`, the four closures are four types. They capture `scale`, so they cannot be coerced
  to the function pointer type `fn(i32, i32) -> i32` either.

The model solution moves the branching **inside** a single value. `select` builds one `filter` whose
closure matches on `keep`, `describe` produces a `String` in both branches, and `op_for` returns one
closure that matches on `symbol` each time it is called. These closures need `move`: `keep`,
`symbol` and `scale` are parameters, and a closure that only borrowed them could not outlive the
call (E0373, "closure may outlive the current function"). They are `Copy`, so `move` just copies
them.

Two logic bugs hide behind the type errors, and fixing only the types (for example with the
compiler's suggestion `n.to_string()` for the `else` branch) does not remove them:

- `%` is the remainder, and its sign follows the left operand: `-3 % 2` is `-1`. So `x % 2 == 1`
  misses negative odd numbers. Test `x % 2 != 0` instead (or `x.rem_euclid(2) == 1`).
- `-n` overflows for `i32::MIN`, because `2147483648` does not fit in an `i32`. With overflow
  checks on, as in the judge, it panics. `n.abs()` has the same problem. `n.unsigned_abs()` returns
  the absolute value as a `u32`, where it always fits.

### Alternatives

Keep one branch per case and erase the types instead. `Box<dyn Iterator<Item = i32> + '_>` is a
single type whatever iterator it holds, and it implements `Iterator` itself, so it can be returned
as `impl Iterator`. The annotation on `boxed` is what turns each arm into the trait object: without
it, every `Box::new(...)` keeps its own concrete type and the arms still disagree. `+ '_` says that
the iterator borrows `v`. Since the return type of `op_for` may change, it can return
`Box<dyn Fn(i32, i32) -> i32>` directly, and the return type converts each arm.

```rust
pub fn select(v: &[i32], keep: Keep) -> impl Iterator<Item = i32> + '_ {
    let iter = v.iter().copied();
    let boxed: Box<dyn Iterator<Item = i32> + '_> = match keep {
        Keep::Evens => Box::new(iter.filter(|x| x % 2 == 0)),
        Keep::Odds => Box::new(iter.filter(|x| x % 2 != 0)),
        Keep::All => Box::new(iter),
    };
    boxed
}

pub fn op_for(symbol: char, scale: i32) -> Box<dyn Fn(i32, i32) -> i32> {
    match symbol {
        '+' => Box::new(move |a, b| (a + b) * scale),
        '-' => Box::new(move |a, b| (a - b) * scale),
        '*' => Box::new(move |a, b| a * b * scale),
        _ => Box::new(move |a, _| a * scale),
    }
}
```

Boxing costs a heap allocation and an indirect call on every element or call, which the model
solution avoids.

Closures that capture nothing do coerce to function pointers. So `op_for` can pick the operation
as a `fn(i32, i32) -> i32` first and apply `scale` in a single capturing closure.

```rust
pub fn op_for(symbol: char, scale: i32) -> impl Fn(i32, i32) -> i32 {
    let base: fn(i32, i32) -> i32 = match symbol {
        '+' => |a, b| a + b,
        '-' => |a, b| a - b,
        '*' => |a, b| a * b,
        _ => |a, _| a,
    };
    move |a, b| base(a, b) * scale
}
```
