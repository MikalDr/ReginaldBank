use crate::ast::base::money::Denomination;

pub mod env;
pub mod money;

#[derive(Debug)]
pub enum ReginaldAST {
    Funds,
    FundsChange(Expr),
    FundsCalc(Expr),
    Calc(Expr),
    Roll(usize, usize),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Val(i64, Denomination),
    Lit(f32),
    Die(usize, usize),
}
