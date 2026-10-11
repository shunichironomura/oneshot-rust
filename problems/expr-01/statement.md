An expression in one variable `x` is a tree of `Expr` values (the enum is already in the editor).
Implement `simplify`, which rewrites an expression with a few algebraic rules, and `eval`, which
computes its value.

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
pub fn eval(e: &Expr, x: i64) -> Option<i64>
```

Below, `Expr::` and `Box::new` are left out: `Add(Var, Num(1))` stands for
`Expr::Add(Box::new(Expr::Var), Box::new(Expr::Num(1)))`.

`simplify` works bottom-up: it first simplifies the children of a node, then rewrites the node (now
with simplified children) by the table below. `a` and `b` stand for numbers, `e` and `f` for any
expressions.

| Node | Becomes |
| --- | --- |
| `Add(Num(a), Num(b))` | `Num(a + b)` |
| `Add(e, Num(0))` or `Add(Num(0), e)` | `e` |
| `Add(e, Neg(f))` or `Add(Neg(f), e)`, where `e == f` | `Num(0)` |
| `Mul(Num(a), Num(b))` | `Num(a * b)` |
| `Mul(e, Num(0))` or `Mul(Num(0), e)` | `Num(0)` |
| `Mul(e, Num(1))` or `Mul(Num(1), e)` | `e` |
| `Neg(Num(a))` | `Num(-a)` |
| `Neg(Neg(e))` | `e` |

- If `a + b`, `a * b` or `-a` overflows `i64`, that rule does not apply.
- A node that no rule applies to stays as it is, with its simplified children. `Num` and `Var` never
  change.
- `e == f` compares the two subtrees exactly, as `==` on `Expr` does. Nothing is ever reordered or
  regrouped: `Mul(Var, Num(2))` and `Mul(Num(2), Var)` are different, and
  `Add(Add(Var, Num(1)), Num(2))` stays as it is.
- When several rules apply to a node, they all give the same result.

`eval` returns the value of `e` when `Var` is `x`. It returns `None` if any addition, multiplication
or negation in the expression overflows `i64`, even in a part that does not affect the result (such
as a factor of a product with `0`).

### Examples

- `simplify(Add(Mul(Var, Num(1)), Num(0)))` is `Var`.
- `simplify(Mul(Add(Num(2), Num(3)), Var))` is `Mul(Num(5), Var)`.
- `simplify(Add(Mul(Var, Num(0)), Num(7)))` is `Num(7)`: the product becomes `Num(0)` first.
- `simplify(Neg(Num(i64::MIN)))` is `Neg(Num(i64::MIN))`, because `-i64::MIN` overflows.
- `simplify(Add(Var, Neg(Mul(Var, Num(1)))))` is `Num(0)`.
- `eval(&Add(Mul(Var, Var), Num(1)), 3)` is `Some(10)`, and `eval(&Neg(Var), i64::MIN)` is `None`.
