//! Reginald Bank

#[cfg(feature = "discord")]
use anyhow::Context;
use anyhow::Result;
#[cfg(feature = "discord")]
use reginald_conf::ReginaldConfig;
#[cfg(feature = "discord")]
use reginald_discord::{run_bot, DiscordHandle};
#[cfg(feature = "discord")]
use std::env;

#[cfg(feature = "discord")]
const DISCORD_TOKEN_ENV: &str = "DISCORD_TOKEN";

#[tokio::main]
async fn main() -> Result<()> {
    if cfg!(feature = "discord") && cfg!(feature = "repl") {
        panic!("Cant have both discord and repl feature at the same time");
    }

    #[cfg(feature = "discord")]
    discord().await?;

    #[cfg(feature = "repl")]
    repl()?;

    Ok(())
}

#[cfg(feature = "discord")]
async fn discord() -> Result<()> {
    let handler = DiscordHandle::new()?;
    handler.setup(ReginaldConfig::get_conf()?)?;

    let token = env::var(DISCORD_TOKEN_ENV)
        .with_context(|| format!("Missing environment variable: {DISCORD_TOKEN_ENV}"))?;

    run_bot(handler, token).await
}

#[cfg(feature = "repl")]
fn repl() -> Result<()> {
    reginald_ast::repl::repl()
}
