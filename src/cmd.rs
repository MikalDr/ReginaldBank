use anyhow::{Error, Result};
use pest::Parser;
use pest_derive::Parser;
use serenity::model::channel::Message;
use serenity::prelude;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CmdParser;

/// Parses commands, will assume that any message given should be a valid command.
pub async fn command_parser(_ctx: prelude::Context, msg: Message) -> Result<()> {
    // TODO: Find out why the parser returns an iterator, does it mean a user can send int multiple
    // commands?
    let input = CmdParser::parse(Rule::input, &msg.content)?
        .into_iter()
        .next()
        .ok_or(Error::msg("Expected `input`, got None"))?;

    // We will just assume that `input` is valid, and get the inner-pairs
    // Since `input` is `bot` - `command`, the inner of the input will be:
    // bot and then command
    let mut inner_iter = input.into_inner();
    // We don't really need the `bot` pair, unless we want to do different things
    // based on what the user called regi
    let _ = inner_iter.next();

    // We will just assume it's a `command`-pair
    let mut command_iter = inner_iter
        .next()
        .ok_or(Error::msg("Expected a command, got None"))?
        .into_inner();

    let cmd = command_iter
        .next()
        .ok_or(Error::msg("Expected a `*_cmd`, got None"))?;

    match cmd.as_rule() {
        Rule::log_cmd => todo!(),
        Rule::add_cmd => todo!(),
        Rule::funds_cmd => todo!(),
        Rule::bag_cmd => todo!(),
        Rule::find_cmd => todo!(),
        Rule::take_cmd => todo!(),
        _ => todo!("Only a command rule should occur"),
    }

    Ok(())
}
