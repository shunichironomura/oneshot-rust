An expression in one variable `x` is a tree of `Expr` values (the enum is already in the editor).
Implement `simplify`, which rewrites an expression with a few algebraic rules.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i64),                  // a constant
    Var,                       // the variable x
    Add(Box<Expr>, Box<Expr>), // left + right
    Mul(Box<Expr>, Box<Expr>), // left * right
    Neg(Box<Expr>),            // -operand
}

pub fn simplify(e: Expr) -> Expr
```

Below, `Expr::` and `Box::new` are left out: `Add(Var, Num(1))` stands for
`Expr::Add(Box::new(Expr::Var), Box::new(Expr::Num(1)))`.

`simplify` works bottom-up: it first simplifies the children of a node, then rewrites the node (now
with simplified children) by the table below. `e` and `f` stand for any expressions.

| Node | Becomes |
| --- | --- |
| `Add(e, Num(0))` or `Add(Num(0), e)` | `e` |
| `Add(e, Neg(f))` or `Add(Neg(f), e)`, where `e == f` | `Num(0)` |
| `Mul(e, Num(0))` or `Mul(Num(0), e)` | `Num(0)` |
| `Mul(e, Num(1))` or `Mul(Num(1), e)` | `e` |
| `Neg(Neg(e))` | `e` |

- A node that no rule applies to stays as it is, with its simplified children. `Num` and `Var` never
  change, and numbers are never added or multiplied: `Add(Num(2), Num(3))` stays as it is.
- `e == f` compares the two subtrees exactly, as `==` on `Expr` does. Nothing is ever reordered or
  regrouped: `Mul(Var, Num(2))` and `Mul(Num(2), Var)` are different, and
  `Add(Add(Var, Num(1)), Num(2))` stays as it is.
- When several rows match a node, the first one from the top is used.

### Examples

- `simplify(Add(Mul(Var, Num(1)), Num(0)))` is `Var`.
- `simplify(Neg(Neg(Add(Var, Num(0)))))` is `Var`.
- `simplify(Add(Mul(Var, Num(0)), Num(7)))` is `Num(7)`: the product becomes `Num(0)` first.
- `simplify(Add(Var, Neg(Mul(Var, Num(1)))))` is `Num(0)`.
