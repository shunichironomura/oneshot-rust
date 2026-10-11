use crate::judge::{case, check_eq};
use crate::user::Expr::{self, Num, Var};
use crate::user::{eval, simplify};

const MAX: i64 = i64::MAX;
const MIN: i64 = i64::MIN;

fn add(l: Expr, r: Expr) -> Expr {
    Expr::Add(Box::new(l), Box::new(r))
}

fn mul(l: Expr, r: Expr) -> Expr {
    Expr::Mul(Box::new(l), Box::new(r))
}

fn neg(e: Expr) -> Expr {
    Expr::Neg(Box::new(e))
}

/// No rule applies anywhere in `e`, so `simplify` must return it as it is.
fn unchanged(e: Expr) -> Result<(), String> {
    check_eq(simplify(e.clone()), e)
}

pub fn run() {
    case("sample-1", || {
        check_eq(simplify(add(mul(Var, Num(1)), Num(0))), Var)?;
        check_eq(simplify(mul(add(Num(2), Num(3)), Var)), mul(Num(5), Var))?;
        check_eq(simplify(add(mul(Var, Num(0)), Num(7))), Num(7))
    });
    case("sample-2", || {
        check_eq(simplify(neg(Num(MIN))), neg(Num(MIN)))?;
        check_eq(simplify(add(Var, neg(mul(Var, Num(1))))), Num(0))
    });
    case("sample-3", || {
        check_eq(eval(&add(mul(Var, Var), Num(1)), 3), Some(10))?;
        check_eq(eval(&neg(Var), MIN), None)
    });
    case("hidden-1", || {
        // Nodes that match no rule stay as they are; nothing is reordered or regrouped.
        unchanged(Var)?;
        unchanged(Num(-5))?;
        unchanged(add(Var, Num(2)))?;
        unchanged(add(Var, Var))?;
        unchanged(mul(Num(-1), Var))?;
        unchanged(neg(Var))?;
        unchanged(add(neg(Var), neg(Var)))?;
        unchanged(add(add(Var, Num(1)), Num(2)))?;
        unchanged(mul(Num(2), mul(Num(3), Var)))
    });
    case("hidden-2", || {
        // The rules for 0 and 1 work on either side.
        check_eq(simplify(add(Num(0), Var)), Var)?;
        check_eq(simplify(add(neg(Var), Num(0))), neg(Var))?;
        check_eq(simplify(mul(Num(1), neg(Var))), neg(Var))?;
        check_eq(simplify(mul(add(Var, Num(4)), Num(1))), add(Var, Num(4)))?;
        check_eq(simplify(mul(Num(0), mul(Var, Var))), Num(0))?;
        check_eq(simplify(mul(add(Var, Num(1)), Num(0))), Num(0))
    });
    case("hidden-3", || {
        // Children are simplified first, and the node is then rewritten.
        check_eq(simplify(add(add(add(Var, Num(0)), Num(0)), Num(0))), Var)?;
        check_eq(simplify(add(Num(2), mul(Num(3), Num(4)))), Num(14))?;
        check_eq(simplify(mul(Num(1), add(neg(Num(3)), Num(3)))), Num(0))?;
        check_eq(simplify(neg(neg(add(Var, Num(0))))), Var)?;
        check_eq(simplify(add(mul(neg(Num(1)), Var), Num(0))), mul(Num(-1), Var))?;
        check_eq(simplify(mul(neg(neg(Num(1))), add(Var, mul(Num(2), Num(0))))), Var)
    });
    case("hidden-4", || {
        // A computation that overflows leaves its node as it is.
        unchanged(add(Num(MAX), Num(1)))?;
        unchanged(add(Num(MIN), Num(-1)))?;
        unchanged(mul(Num(MIN), Num(-1)))?;
        unchanged(mul(Num(1 << 32), Num(1 << 31)))?;
        unchanged(add(add(Num(MAX), Num(1)), Num(-1)))?;
        check_eq(simplify(mul(Num(1 << 32), Num(1 << 30))), Num(1 << 62))?;
        check_eq(simplify(mul(Num(-(1 << 32)), Num(1 << 31))), Num(MIN))?;
        check_eq(simplify(add(Num(MAX), Num(MIN))), Num(-1))?;
        check_eq(simplify(neg(Num(MAX))), Num(-MAX))?;
        check_eq(simplify(neg(neg(Num(MIN)))), Num(MIN))?;
        check_eq(simplify(mul(add(Num(MAX), Num(1)), Num(0))), Num(0))?;
        check_eq(simplify(add(Num(0), add(Num(MAX), Num(1)))), add(Num(MAX), Num(1)))
    });
    case("hidden-5", || {
        // e + (-e) and (-e) + e become 0 only when both sides are equal; -(-e) becomes e.
        check_eq(simplify(add(neg(Var), Var)), Num(0))?;
        check_eq(simplify(add(neg(add(Var, Num(1))), add(Var, Num(1)))), Num(0))?;
        check_eq(simplify(add(Num(MIN), neg(Num(MIN)))), Num(0))?;
        check_eq(simplify(add(neg(Num(MIN)), Num(MIN))), Num(0))?;
        check_eq(simplify(add(neg(Var), neg(neg(Var)))), Num(0))?;
        check_eq(simplify(add(Var, neg(neg(Var)))), add(Var, Var))?;
        check_eq(simplify(neg(neg(neg(Var)))), neg(Var))?;
        unchanged(add(mul(Var, Num(2)), neg(mul(Num(2), Var))))?;
        unchanged(add(Var, neg(add(Var, Num(1)))))
    });
    case("hidden-6", || {
        check_eq(eval(&Var, -7), Some(-7))?;
        check_eq(eval(&Num(42), 0), Some(42))?;
        check_eq(eval(&add(mul(Num(3), Var), neg(Num(5))), 4), Some(7))?;
        check_eq(eval(&mul(Var, Var), 3_037_000_499), Some(9_223_372_030_926_249_001))?;
        check_eq(eval(&mul(Var, Var), 3_037_000_500), None)?;
        check_eq(eval(&add(Var, Num(-1)), MIN), None)?;
        check_eq(eval(&add(Num(MAX), Num(MIN)), 0), Some(-1))?;
        check_eq(eval(&neg(Num(MIN)), 0), None)?;
        check_eq(eval(&neg(neg(Var)), MIN), None)?;
        // An overflow anywhere gives `None`, even in a factor of a product with 0.
        check_eq(eval(&mul(Num(0), add(Num(MAX), Num(1))), 0), None)?;
        check_eq(eval(&mul(add(Var, Num(1)), Num(0)), MAX), None)
    });
    case("hidden-7", || {
        // Larger expressions where the rules interact.
        unchanged(add(Var, add(mul(Num(MIN), Num(-1)), neg(mul(Num(-1), Num(MIN))))))?;
        check_eq(
            simplify(add(add(neg(mul(Var, Var)), mul(Var, Var)), neg(add(Var, Num(MIN))))),
            neg(add(Var, Num(MIN))),
        )?;
        check_eq(
            simplify(add(
                mul(add(Var, Num(0)), add(Num(MIN), Num(0))),
                add(mul(Num(-1), Num(MIN)), mul(Num(0), Num(MAX))),
            )),
            add(mul(Var, Num(MIN)), mul(Num(-1), Num(MIN))),
        )?;
        check_eq(
            simplify(add(
                add(Num(1), Var),
                add(add(neg(Num(MIN)), Num(MIN)), neg(add(Num(MIN), neg(Num(MIN))))),
            )),
            add(Num(1), Var),
        )?;
        check_eq(
            simplify(add(
                neg(add(neg(Var), add(neg(Num(3)), Num(3)))),
                add(neg(Var), add(neg(Num(3)), Num(3))),
            )),
            Num(0),
        )?;
        check_eq(
            simplify(mul(
                add(mul(Var, Num(1)), mul(Var, Num(-1))),
                mul(Num(1), add(Num(MAX), Num(MIN))),
            )),
            mul(add(Var, mul(Var, Num(-1))), Num(-1)),
        )
    });
}
