use crate::ast::{
    base::{Expr, money::Denomination},
    typechecker::typed_ast::BinOp,
};
use anyhow::{Result, anyhow};

pub mod base;
pub mod eval;
pub mod rules;
pub mod typechecker;

fn denomination_expr(expr: &Expr) -> Result<Denomination> {
    match expr {
        Expr::Add(left_expr, right_expr)
        | Expr::Sub(left_expr, right_expr)
        | Expr::Mul(left_expr, right_expr)
        | Expr::Div(left_expr, right_expr) => {
            let left_denomination = denomination_expr(left_expr)?;
            let right_denomination = denomination_expr(right_expr)?;

            if left_denomination == right_denomination {
                Ok(left_denomination)
            } else {
                Err(anyhow!(
                    "Missmatch on denomination: ({left_denomination:?}): {left_expr:?} {right_expr:?} ({right_denomination:?}):"
                ))
            }
        }
        Expr::Neg(sub_expr) => denomination_expr(sub_expr),
        Expr::Val(_, denomination) => Ok(*denomination),
        Expr::Lit(_) => Ok(Denomination::default()),
        Expr::Die(_, _) => todo!(),
    }
}

fn get_binop(expr: &Expr) -> Result<BinOp> {
    match expr {
        Expr::Add(_, _) => Ok(BinOp::Add),
        Expr::Sub(_, _) => Ok(BinOp::Sub),
        Expr::Mul(_, _) => Ok(BinOp::Mult),
        Expr::Div(_, _) => Ok(BinOp::Div),
        _ => Err(anyhow!("Not a binary operation")),
    }
}
