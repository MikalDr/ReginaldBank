use crate::ast::{
    base::{Expr, ReginaldAST},
    denomination_expr, get_binop,
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
        Expr::Neg(expr) => Ok(TypedExpr::Neg(Box::new(typecheck_expr(*expr.clone())?))),
        Expr::Val(val, denomination) => Ok(TypedExpr::Val(*val, *denomination)),
        Expr::Add(l, r) | Expr::Sub(l, r) | Expr::Mul(l, r) | Expr::Div(l, r) => {
            let binop = get_binop(&expr)?;
            let left_denomination = denomination_expr(l.as_ref())?;
            let right_denomination = denomination_expr(r.as_ref())?;
            Ok(TypedExpr::BinOp(
                binop,
                Box::new(typecheck_expr(*l.clone())?),
                Box::new(typecheck_expr(*r.clone())?),
                left_denomination.min(right_denomination),
            ))
        }
        Expr::Lit(val) => Ok(TypedExpr::Lit(*val)),
        Expr::Die(_, _) => todo!(),
    }
}
