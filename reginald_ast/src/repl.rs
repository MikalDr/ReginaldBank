use crate::{
    ReginaldParser, Rule,
    ast::{base::env::ReginaldEnv, eval::eval, rules::rules_to_ast, typechecker::typecheck},
};
use anyhow::Result;
use pest::Parser;
use std::io::{self, Write};

pub fn repl() -> Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut env = ReginaldEnv::default();
    loop {
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut buffer)?;
        match foobar(buffer.trim(), &env) {
            Ok(new_env) => {
                env = new_env;
            }
            Err(err) => {
                let dbg_cmd = buffer.trim().to_ascii_lowercase();
                match dbg_cmd.as_str() {
                    "q!" | "q" => break,
                    "r" => env = ReginaldEnv::default(),
                    "dbg" => println!("{env:?}"),
                    _ => eprintln!("> Error: {err:?}"),
                }
            }
        }
        buffer.clear();
    }

    Ok(())
}

fn foobar(input: &str, env: &ReginaldEnv) -> Result<ReginaldEnv> {
    let pairs = ReginaldParser::parse(Rule::program, input)?;
    let ast = rules_to_ast(pairs)?;
    let typed_ast = typecheck(ast)?;
    eval(typed_ast, env.clone())
}
