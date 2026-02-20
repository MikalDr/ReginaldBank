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
    Help,
}

#[derive(Debug, Clone)]
pub enum Expr {
    BinOp(BinOp, Box<Self>, Box<Self>),
    UniOp(UniOp, Box<Self>),
    Val(i64, Denomination),
    Lit(f32),
    Die(usize, usize),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum UniOp {
    Neg,
    Abs,
}
