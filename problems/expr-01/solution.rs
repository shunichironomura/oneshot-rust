#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A constant.
    Num(i64),
    /// The variable `x`.
    Var,
    /// `left + right`
    Add(Box<Expr>, Box<Expr>),
    /// `left * right`
    Mul(Box<Expr>, Box<Expr>),
    /// `-operand`
    Neg(Box<Expr>),
}

pub fn simplify(e: Expr) -> Expr {
    match e {
        Expr::Num(_) | Expr::Var => e,
        // `*l` moves the `Expr` out of its box; the children are simplified first.
        Expr::Add(l, r) => simplify_add(simplify(*l), simplify(*r)),
        Expr::Mul(l, r) => simplify_mul(simplify(*l), simplify(*r)),
        Expr::Neg(inner) => simplify_neg(simplify(*inner)),
    }
}

fn simplify_add(l: Expr, r: Expr) -> Expr {
    match (l, r) {
        (e, Expr::Num(0)) | (Expr::Num(0), e) => e,
        (e, Expr::Neg(f)) | (Expr::Neg(f), e) if e == *f => Expr::Num(0),
        (l, r) => Expr::Add(Box::new(l), Box::new(r)),
    }
}

fn simplify_mul(l: Expr, r: Expr) -> Expr {
    match (l, r) {
        (_, Expr::Num(0)) | (Expr::Num(0), _) => Expr::Num(0),
        (e, Expr::Num(1)) | (Expr::Num(1), e) => e,
        (l, r) => Expr::Mul(Box::new(l), Box::new(r)),
    }
}

fn simplify_neg(e: Expr) -> Expr {
    match e {
        Expr::Neg(inner) => *inner,
        e => Expr::Neg(Box::new(e)),
    }
}
