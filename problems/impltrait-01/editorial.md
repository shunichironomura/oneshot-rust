A function that returns `impl Trait` still returns **one** concrete type. The body picks that type,
and callers only see "some type that implements the trait". It does not mean "any value that
implements the trait", so every branch has to produce the same type. Every closure has its own
anonymous type, even when two closures look the same. So `Filter<Copied<Iter<i32>>, {closure#1}>`
and `Filter<Copied<Iter<i32>>, {closure#2}>` are different types, and `Copied<Iter<i32>>` is a third
one.

The model solution moves the branching **inside** a single value: it builds one `filter` whose
closure matches on `keep`. That closure needs `move`: `keep` is a parameter, and a closure that only
borrowed it could not outlive the call. Without `move`, rustc reports E0373 ("closure may outlive
the current function"). `Keep` is `Copy`, so `move` just copies it.

A logic bug hides behind the type error, and fixing only the types does not remove it. `%` is the
remainder, and its sign follows the left operand: `-3 % 2` is `-1`. So `x % 2 == 1` misses negative
odd numbers. Test `x % 2 != 0` instead (or `x.rem_euclid(2) == 1`).

### Alternatives

Keep one branch per case and erase the types instead. `Box<dyn Iterator<Item = i32> + '_>` is a
single type whatever iterator it holds, and it implements `Iterator` itself, so it can be returned
as `impl Iterator`. The annotation on `boxed` is what turns each arm into the trait object: without
it, every `Box::new(...)` keeps its own concrete type and the arms still disagree. `+ '_` says that
the iterator borrows `v`.

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
```

Boxing costs a heap allocation and an indirect call on every element, which the model solution
avoids.
