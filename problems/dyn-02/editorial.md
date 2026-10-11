In a type, `Box<dyn Trait>` without a lifetime means `Box<dyn Trait + 'static>`: that is the default
lifetime bound of a trait object in a `Box`. So the field `Vec<Box<dyn Fn(&str) -> bool>>` can only
hold rules that borrow nothing (or only `'static` data), and `add` fails with E0310 because `rule`
might borrow something. The compiler suggests `impl Fn(&str) -> bool + 'static`. That makes `add`
compile, but then no closure that borrows a local variable can be added, and `banning`, whose
closure borrows `words`, fails in turn.

To let the rules borrow, give `Validator` a lifetime parameter and use it as the bound of the trait
object. `Box<dyn Fn(&str) -> bool + 'a>` is "a boxed rule that may only borrow things that live at
least as long as `'a`". The `impl` block becomes `impl<'a> Validator<'a>`, so that the methods can
name `'a`:

- `add` takes `impl Fn(&str) -> bool + 'a`.
- `add_boxed` takes `Box<dyn Fn(&str) -> bool + 'a>`. Left unchanged, it still compiles but only
  accepts `'static` rules, and the harness boxes a closure that borrows `suffix`. A plain
  `Box<dyn Fn(&str) -> bool>` is still accepted, because `'static` outlives `'a`.
- `banning` returns `Validator<'_>`, a validator that borrows `words`.

Only the trait object gets `'a`, not the argument of the rule. `dyn Fn(&str) -> bool` is short for
the higher-ranked `dyn for<'x> Fn(&'x str) -> bool`: it accepts a string of any lifetime. With
`dyn Fn(&'a str) -> bool + 'a`, `check` could only be given strings that live as long as the borrows
of the rules, and the harness checks strings that are created after the validator.

Finally, the elided lifetime in the result of `passing` comes from `&self`, but the strings it
returns are the ones in `items`, not anything inside the validator. Give them their own lifetime:
`passing<'s>(&self, items: &[&'s str]) -> Vec<&'s str>`. The compiler suggests the name `'a` for it,
but inside `impl<'a>` that name is already taken (E0496), so pick another one. With `&'s self`, the
result would keep the validator borrowed, so `drop(v)` fails. With `&'s [&'s str]`, it would borrow
the slice, which is a temporary `Vec` in one case. With `&'a str`, it could not outlive the
variables that the rules borrow.

### Alternatives

`add` can take a generic type parameter instead of `impl Trait`, and `banning` can name its
lifetime.

```rust
pub fn add<F>(&mut self, rule: F)
where
    F: Fn(&str) -> bool + 'a,
{
    self.rules.push(Box::new(rule));
}
```

```rust
pub fn banning<'w>(words: &'w [String]) -> Validator<'w> {
    let mut v = Validator::new();
    v.add(move |s| !words.iter().any(|w| s.contains(w.as_str())));
    v
}
```
