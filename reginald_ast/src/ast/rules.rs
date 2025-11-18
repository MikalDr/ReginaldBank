use crate::{
    Rule,
    ast::base::{Expr, ReginaldAST, money::Denomination},
};
use anyhow::{Result, anyhow};
use pest::iterators::Pairs;

pub fn rules_to_ast(mut pairs: Pairs<Rule>) -> Result<ReginaldAST> {
    match pairs.peek() {
        None => Err(anyhow!("No rules found")),
        Some(pair_rule) => match pair_rule.as_rule() {
            Rule::unit | Rule::program | Rule::command => rules_to_ast(pair_rule.into_inner()),
            Rule::funds => {
                let removes = &pairs.peek().is_some_and(|p| p.as_str().contains("take"));
                let _ = pairs.next();
                match pairs.next().map(|pair| pair.into_inner()) {
                    Some(expr) if *removes => Ok(ReginaldAST::FundsChange(Expr::Neg(Box::new(rules_to_expr(expr)?)))),
                    Some(expr) => Ok(ReginaldAST::FundsChange(rules_to_expr(expr)?)),
                    None => Ok(ReginaldAST::Funds),
                }
            }
            Rule::calc => {
                let _ = pairs.next();
                let expr = rules_to_expr(pairs)?;
                Ok(ReginaldAST::Calc(expr))
            }
            Rule::help => Ok(ReginaldAST::Help),
            Rule::roll => {
                let pair = pairs.next().unwrap();
                let (min, max) = parse_dice_rule(pair.into_inner());
                Ok(ReginaldAST::Roll(min, max))
            }
            #[allow(unreachable_patterns)]
            _ => {
                dbg!(&pairs);
                panic!("Unhandled rule: {:?}", pairs.as_str())
            }
        },
    }
}

fn parse_dice_rule(mut pairs: Pairs<Rule>) -> (usize, usize) {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::dice => match pair.as_str() {
            "d2" => (1, 2),
            "d4" => (1, 4),
            "d6" => (1, 6),
            "d8" => (1, 8),
            "d10" => (1, 10),
            "d12" => (1, 12),
            "d20" => (1, 20),
            "d100" => (1, 100),
            _ => panic!("Unexpected die: {pairs:?}"),
        },
        _ => panic!("Unexpected rule on parse dice: {pairs:?}"),
    }
}

pub fn rules_to_expr(mut pairs: Pairs<Rule>) -> Result<Expr> {
    match pairs.peek() {
        None => Err(anyhow!("No expr rules found")),
        Some(pair_rule) => match pair_rule.as_rule() {
            Rule::money => {
                let mut inner = pair_rule.into_inner();

                let val = inner.next().unwrap().as_str().parse::<i64>()?;

                let denomination = inner.next().unwrap().as_str().parse::<Denomination>()?;

                let left = Expr::Val(val, denomination);

                let _ = pairs.next();

                match pairs.next().map(|p| p.as_rule()) {
                    Some(Rule::add) => {
                        Ok(Expr::Add(Box::new(left), Box::new(rules_to_expr(pairs)?)))
                    }
                    Some(Rule::subtract) => {
                        Ok(Expr::Sub(Box::new(left), Box::new(rules_to_expr(pairs)?)))
                    }
                    Some(Rule::multiply) => {
                        Ok(Expr::Mul(Box::new(left), Box::new(rules_to_expr(pairs)?)))
                    }
                    Some(Rule::divide) => {
                        Ok(Expr::Div(Box::new(left), Box::new(rules_to_expr(pairs)?)))
                    }
                    None => Ok(left),
                    _ => panic!("Unhandled operator rule: {:?}", pairs),
                }
            }
            Rule::expr => {
                let mut pairs = pairs.next().unwrap().into_inner();
                let left = rules_to_expr(pairs.next().unwrap().into_inner())?;
                match (pairs.next().map(|p| p.as_rule()), pairs.next()) {
                    (None, None) => Ok(left),
                    (Some(Rule::add), Some(right)) => Ok(Expr::Add(
                        Box::new(left),
                        Box::new(rules_to_expr(right.into_inner())?),
                    )),
                    (Some(Rule::subtract), Some(right)) => Ok(Expr::Sub(
                        Box::new(left),
                        Box::new(rules_to_expr(right.into_inner())?),
                    )),
                    (Some(Rule::multiply), Some(right)) => Ok(Expr::Mul(
                        Box::new(left),
                        Box::new(rules_to_expr(right.into_inner())?),
                    )),
                    (Some(Rule::divide), Some(right)) => Ok(Expr::Div(
                        Box::new(left),
                        Box::new(rules_to_expr(right.into_inner())?),
                    )),
                    _ => panic!("Unhandled operator rule: {:?}", pairs),
                }
            }
            Rule::num => {
                let lit = pairs.next().unwrap().as_str().parse::<f32>()?;
                Ok(Expr::Lit(lit))
            }
            Rule::lit => {
                let left = rules_to_expr(pairs.next().unwrap().into_inner())?;
                match (pairs.next().map(|p| p.as_rule()), pairs.next()) {
                    (None, None) => Ok(left),
                    (Some(Rule::add), Some(right)) => Ok(Expr::Add(
                        Box::new(left),
                        Box::new(rules_to_expr(right.into_inner())?),
                    )),
                    (Some(Rule::subtract), Some(right)) => Ok(Expr::Sub(
                        Box::new(left),
                        Box::new(rules_to_expr(right.into_inner())?),
                    )),
                    (Some(Rule::multiply), Some(right)) => Ok(Expr::Mul(
                        Box::new(left),
                        Box::new(rules_to_expr(right.into_inner())?),
                    )),
                    (Some(Rule::divide), Some(right)) => Ok(Expr::Div(
                        Box::new(left),
                        Box::new(rules_to_expr(right.into_inner())?),
                    )),
                    _ => panic!("Unhandled operator rule: {:?}", pairs),
                }
            }
            #[allow(unreachable_patterns)]
            _ => panic!("Unhandled expr rule: {:?}", pairs),
        },
    }
}
