use crate::ast::base::env::ReginaldEnv;
use crate::ast::eval::eval;
use crate::ast::rules::rules_to_ast;
use crate::ast::typechecker::typecheck;
use crate::{ReginaldParser, Rule};
use pest::Parser;

mod piece_test;

const TEST_FUNDS_EXAMPLE: &str = include_str!("../../.././grammar/examples/funds-example");
const TEST_EVAL_EXAMPLE: &str = include_str!("../../.././grammar/examples/eval-example");

#[test]
fn test_parser_funds_example() {
    for line in TEST_FUNDS_EXAMPLE.lines() {
        let cmd = ReginaldParser::parse(Rule::program, line);
        assert!(cmd.is_ok(), "{:?}", cmd);
    }
}

#[test]
fn test_ast_funds_example() {
    for line in TEST_FUNDS_EXAMPLE.lines() {
        let cmd = ReginaldParser::parse(Rule::program, line);
        let ast = rules_to_ast(cmd.unwrap());
        assert!(ast.is_ok(), "{:?}", ast);
    }
}

#[test]
fn test_typed_ast_funds_example() {
    for line in TEST_FUNDS_EXAMPLE.lines() {
        let cmd = ReginaldParser::parse(Rule::program, line);
        let ast = rules_to_ast(cmd.unwrap());
        let typed_ast = typecheck(ast.unwrap());
        assert!(typed_ast.is_ok(), "{:?}", typed_ast);
    }
}

#[test]
fn test_eval_typed_ast_funds_example() {
    for line in TEST_FUNDS_EXAMPLE.lines() {
        let cmd = ReginaldParser::parse(Rule::program, line);
        let ast = rules_to_ast(cmd.unwrap());
        let typed_ast = typecheck(ast.unwrap()).unwrap();
        let env = ReginaldEnv::default();
        let new_env = eval(typed_ast, env);
        assert!(new_env.is_ok(), "{:?}", new_env);
    }
}

#[test]
fn test_eval_typed_ast_funds() {
    let cmd = ReginaldParser::parse(Rule::program, "funds add 19cp + 30 cp");
    let ast = rules_to_ast(cmd.unwrap());
    let typed_ast = typecheck(ast.unwrap()).unwrap();
    let env = ReginaldEnv::default();
    let new_env = eval(typed_ast, env).unwrap();
    assert_eq!(new_env.cp, 19 + 30, "{:?}", new_env);
}

#[test]
fn test_eval_typed_ast_calc_example() {
    for line in TEST_EVAL_EXAMPLE.lines() {
        let cmd = ReginaldParser::parse(Rule::program, line);
        let ast = rules_to_ast(cmd.unwrap());
        let typed_ast = typecheck(ast.unwrap()).unwrap();
        let env = ReginaldEnv::default();
        let new_env = eval(typed_ast, env);
        assert!(new_env.is_ok(), "{:?}", new_env);
    }
}
