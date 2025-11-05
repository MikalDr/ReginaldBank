use crate::{
    ReginaldParser, Rule,
    ast::{
        base::env::ReginaldEnv,
        rules::rules_to_ast,
        typechecker::{
            typecheck,
            typed_ast::{BinOp, TypedAST, TypedExpr, Value},
        },
    },
};
use anyhow::Result;
use pest::Parser;
use rand::{rng, seq::IteratorRandom};
use std::ops::Neg;

pub fn evaluate(msg: &str, env: ReginaldEnv) -> Result<ReginaldEnv> {
    let pairs = ReginaldParser::parse(Rule::program, msg)?;
    let ast = rules_to_ast(pairs)?;
    let typed_ast = typecheck(ast)?;
    eval(typed_ast, env.clone())
}

pub fn eval(ast: TypedAST, env: ReginaldEnv) -> Result<ReginaldEnv> {
    match ast {
        TypedAST::FundsChange(expr) => {
            let value = eval_expr(expr, &env)?;
            Ok(env.add(value))
        }
        TypedAST::Funds => {
            let funds = env.funds();
            Ok(env.print(funds))
        }
        TypedAST::Calc(expr) => {
            let result = eval_expr(expr, &env)?;
            Ok(env.print(format!("Calc: {:?}", result.get_value())))
        }
        _ => panic!("No eval for {ast:?}"),
    }
}

fn eval_expr(expr: TypedExpr, env: &ReginaldEnv) -> Result<Value> {
    match expr {
        TypedExpr::BinOp(op, l, r, _) => match op {
            BinOp::Add | BinOp::Sub => {
                let op: fn(f32, f32) -> f32 = match op {
                    BinOp::Add => |a, b| a + b,
                    BinOp::Sub => |a, b| a - b,
                    BinOp::Mult | BinOp::Div => unreachable!(),
                };

                let left = eval_expr(*l, env)?;
                let right = eval_expr(*r, env)?;

                match (left, right) {
                    (Value::Piece(left, left_den), Value::Piece(right, right_den)) => {
                        let target = left_den.min(right_den);
                        let source = left_den.max(right_den);
                        let convert = target.exchange(source);
                        let left_con = convert(left);
                        let right_con = convert(right);
                        let result = op(left_con, right_con);
                        Ok(Value::Piece(result.round() as i64, target))
                    }
                    (Value::Int(val), Value::Piece(piece, den))
                    | (Value::Piece(piece, den), Value::Int(val)) => Ok(Value::Piece(
                        op(val as f32, piece as f32).round() as i64,
                        den,
                    )),
                    (Value::Float(val), Value::Piece(piece, den))
                    | (Value::Piece(piece, den), Value::Float(val)) => {
                        Ok(Value::Piece(op(val, piece as f32).round() as i64, den))
                    }
                    (Value::Int(left), Value::Int(right)) => {
                        Ok(Value::Int(op(left as f32, right as f32).round() as i64))
                    }
                    (Value::Int(left), Value::Float(right)) => {
                        Ok(Value::Float(op(left as f32, right)))
                    }
                    (Value::Float(left), Value::Int(right)) => {
                        Ok(Value::Float(op(left, right as f32)))
                    }
                    (Value::Float(left), Value::Float(right)) => Ok(Value::Float(op(left, right))),
                }
            }
            BinOp::Mult => todo!(),
            BinOp::Div => todo!(),
        },
        TypedExpr::Neg(expr) => {
            let value = eval_expr(*expr, env)?;
            Ok(value.neg())
        }
        TypedExpr::Val(val, denomination) => Ok(Value::Piece(val, denomination)),
        TypedExpr::Lit(val) => Ok(Value::Float(val)),
        TypedExpr::Die(s, e) => {
            let choice = (s..e).choose(&mut rng()).unwrap_or_default();
            Ok(Value::Int(choice as i64))
        }
    }
}
