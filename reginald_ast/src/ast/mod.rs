use crate::ast::{
    base::{BinOp, Expr, UniOp, money::Denomination},
};
use anyhow::{Result, anyhow};

pub mod base;
pub mod eval;
pub mod rules;
pub mod typechecker;

fn denomination_expr(expr: &Expr) -> Result<Denomination> {
    match expr {
        Expr::BinOp(op, left_expr, right_expr) => match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
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
        },
        Expr::UniOp(op, sub_expr) => match op {
            UniOp::Neg => denomination_expr(&sub_expr),
            _ => todo!(),
        },
        Expr::Val(_, denomination) => Ok(*denomination),
        Expr::Lit(_) => Ok(Denomination::default()),
        Expr::Die(_, _) => todo!(),
    }
}
