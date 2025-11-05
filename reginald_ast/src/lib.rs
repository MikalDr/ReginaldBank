//! Reginald Bank

use pest_derive::Parser;

pub mod ast;
#[cfg(feature = "repl")]
pub mod repl;
#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "../grammar/base.pest"]
#[grammar = "../grammar/reginald.pest"]
#[grammar_inline = r#"
command = { funds ~ expr? | calc ~ expr }
program = { SOI ~ command+ ~ EOI }
"#]
pub struct ReginaldParser;
