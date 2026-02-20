use crate::ast::base::env::ReginaldEnv;
use crate::ast::eval::eval;
use crate::ast::rules::rules_to_ast;
use crate::ast::typechecker::typecheck;
use crate::{ReginaldParser, Rule};
use pest::Parser;

const TEST_BASIC_CMDS: &str = include_str!("../../.././grammar/examples/basic-cmds");

#[test]
fn test_basic_commands() {
    for line in TEST_BASIC_CMDS.lines() {
        let cmd = ReginaldParser::parse(Rule::program, line);
        let ast = rules_to_ast(cmd.unwrap());
        let typed_ast = typecheck(ast.unwrap()).unwrap();
        let env = ReginaldEnv::default();
        let new_env = eval(typed_ast, env);
        assert!(new_env.is_ok(), "{:?}", new_env);
    }
}
