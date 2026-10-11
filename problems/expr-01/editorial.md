Stable Rust has no box patterns, so a pattern cannot look inside a `Box`: `Expr::Add(Expr::Num(a), _)`
is a type error, because the field is a `Box<Expr>`. The solution takes the node apart, simplifies
the children first (the bottom-up order needs that anyway), and matches on the tuple of the two
**owned** results. `*l` moves the `Expr` out of the box `l`. Only `Box` allows moving out through
`*`, and the box is unusable afterwards: writing `Expr::Neg(inner)` after `simplify(*inner)` is a
"use of moved value". New nodes are therefore built with `Box::new`. For the same reason, once
`match e` has moved the boxes out of `e`, `e` cannot be returned as a fallback.

Details that are easy to get wrong:

- In an or-pattern, every alternative must bind the same names with the same types.
  `(e, Expr::Num(0)) | (Expr::Num(0), e)` works; `(e, Expr::Num(0)) | (Expr::Num(0), _)` does not
  compile.
- In `(e, Expr::Neg(f))`, `f` is a `Box<Expr>`. `e == f` compares an `Expr` with a `Box<Expr>` and
  does not compile; `e == *f` does. A guard only borrows the bindings, so nothing is moved by the
  comparison. If the guard fails for one alternative of an or-pattern, the next alternative is tried.
- `a + b`, `a * b` and `-a` panic on overflow in the judge (as in any debug build), so use
  `checked_add`, `checked_mul` and `checked_neg`. With an `if let` guard (stable since Rust 1.95),
  an overflow just makes the arm not match. No later arm matches two such numbers either, so the
  last arm rebuilds the node unchanged.
- `Expr::Neg(e) => *e`: the inner `e` is a box, and the result must be the `Expr` inside it.
- Rules are applied after the children are simplified. Checking the rules first and simplifying the
  children afterwards misses `Add(Mul(Var, Num(0)), Num(7))`, whose product only becomes `Num(0)`
  during simplification.

In `eval`, `?` returns `None` as soon as an operand fails, and each `checked_*` call returns `None`
when the operation itself overflows. Returning `Some(0)` early for a factor `0` would be wrong,
because the other factor may overflow.

### Alternatives

Without `if let` guards, handle the overflow inside the arm and rebuild the node by hand.

```rust
pub fn simplify(e: Expr) -> Expr {
    match e {
        Expr::Add(l, r) => match (simplify(*l), simplify(*r)) {
            (Expr::Num(a), Expr::Num(b)) => match a.checked_add(b) {
                Some(sum) => Expr::Num(sum),
                None => Expr::Add(Box::new(Expr::Num(a)), Box::new(Expr::Num(b))),
            },
            (e, Expr::Num(0)) | (Expr::Num(0), e) => e,
            (e, Expr::Neg(f)) | (Expr::Neg(f), e) if e == *f => Expr::Num(0),
            (l, r) => Expr::Add(Box::new(l), Box::new(r)),
        },
        Expr::Mul(l, r) => match (simplify(*l), simplify(*r)) {
            (Expr::Num(a), Expr::Num(b)) => match a.checked_mul(b) {
                Some(product) => Expr::Num(product),
                None => Expr::Mul(Box::new(Expr::Num(a)), Box::new(Expr::Num(b))),
            },
            (_, Expr::Num(0)) | (Expr::Num(0), _) => Expr::Num(0),
            (e, Expr::Num(1)) | (Expr::Num(1), e) => e,
            (l, r) => Expr::Mul(Box::new(l), Box::new(r)),
        },
        Expr::Neg(inner) => match simplify(*inner) {
            Expr::Num(a) => match a.checked_neg() {
                Some(negated) => Expr::Num(negated),
                None => Expr::Neg(Box::new(Expr::Num(a))),
            },
            Expr::Neg(e) => *e,
            e => Expr::Neg(Box::new(e)),
        },
        leaf => leaf,
    }
}
```

The boxes can also be reused. `*l = simplify(*l)` moves the child out of its box and puts the
simplified child back in. The match then looks at the children only through references, and
afterwards either moves one child out (`*l`) or reuses both boxes (`Expr::Add(l, r)`), so a node
that stays as it is needs no new allocation. Here is the `Add` arm; `Mul` and `Neg` work the same
way.

```rust
Expr::Add(mut l, mut r) => {
    *l = simplify(*l);
    *r = simplify(*r);
    match (&*l, &*r) {
        (Expr::Num(a), Expr::Num(b)) if let Some(sum) = a.checked_add(*b) => {
            Expr::Num(sum)
        }
        (_, Expr::Num(0)) => *l,
        (Expr::Num(0), _) => *r,
        (e, Expr::Neg(f)) | (Expr::Neg(f), e) if *e == **f => Expr::Num(0),
        _ => Expr::Add(l, r),
    }
}
```
