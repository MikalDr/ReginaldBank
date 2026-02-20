use crate::ast::{
    base::{BinOp, Expr, ReginaldAST, UniOp},
    denomination_expr,
    typechecker::typed_ast::{TypedAST, TypedExpr},
};
use anyhow::Result;

pub mod typed_ast;

pub fn typecheck(ast: ReginaldAST) -> Result<TypedAST> {
    match ast {
        ReginaldAST::Funds => Ok(TypedAST::Funds),
        ReginaldAST::FundsChange(expr) => {
            let expr = typecheck_expr(expr)?;
            Ok(TypedAST::FundsChange(expr))
        }
        ReginaldAST::FundsCalc(expr) => {
            let expr = typecheck_expr(expr)?;
            Ok(TypedAST::FundsCalc(expr))
        }
        ReginaldAST::Calc(expr) => {
            let expr = typecheck_expr(expr)?;
            Ok(TypedAST::Calc(expr))
        }
        ReginaldAST::Roll(s, e) => Ok(TypedAST::Roll(s, e)),
        ReginaldAST::Help => Ok(TypedAST::Help),
    }
}

fn typecheck_expr(expr: Expr) -> Result<TypedExpr> {
    match &expr {
        Expr::Val(val, denomination) => Ok(TypedExpr::Val(*val, *denomination)),
        Expr::BinOp(op, l , r) => match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                let left_denomination = denomination_expr(l)?;
                let right_denomination = denomination_expr(r)?;
                Ok(TypedExpr::BinOp(
                    op.clone(),
                    Box::new(typecheck_expr(*l.clone())?),
                    Box::new(typecheck_expr(*r.clone())?),
                    left_denomination.min(right_denomination),
                ))
            }
        },
        Expr::UniOp(op, expr) => match op {
            UniOp::Neg => Ok(TypedExpr::Neg(Box::new(typecheck_expr(*expr.clone())?))),
            UniOp::Abs => todo!(),
        },
        Expr::Lit(val) => Ok(TypedExpr::Lit(*val)),
        Expr::Die(_, _) => todo!(),
    }
}
