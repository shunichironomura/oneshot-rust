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
    todo!()
}
