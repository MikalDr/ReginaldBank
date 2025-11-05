use std::ops::Neg;

use crate::ast::base::money::Denomination;

#[derive(Debug)]
pub enum TypedAST {
    Funds,
    FundsChange(TypedExpr),
    FundsCalc(TypedExpr),
    Calc(TypedExpr),
    Roll(usize, usize),
}

#[derive(Debug)]
pub enum TypedExpr {
    BinOp(BinOp, Box<Self>, Box<Self>, Denomination),
    Neg(Box<Self>),
    Val(i64, Denomination),
    Lit(f32),
    Die(usize, usize),
}

impl TypedExpr {
    pub fn type_eq(&self, other: &Self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Mult,
    Sub,
    Div,
}

#[derive(Debug)]
pub enum Value {
    Piece(i64, Denomination),
    Int(i64),
    Float(f32),
}

impl Value {
    pub fn get_value(self) -> f32 {
        match self {
            Value::Piece(val, denomination) => denomination.exchange(Denomination::Gold)(val),
            Value::Int(val) => val as f32,
            Value::Float(val) => val,
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Piece(val, denomination) => Value::Piece(val.neg(), denomination),
            Value::Int(val) => Value::Int(val.neg()),
            Value::Float(val) => Value::Float(val.neg()),
        }
    }
}
