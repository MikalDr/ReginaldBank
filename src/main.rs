//! Reginald Bank

use anyhow::{Context, Result};
use reginald_conf::ReginaldConfig;
use reginald_discord::{run_bot, DiscordHandle};
use std::env;

const DISCORD_TOKEN_ENV: &str = "DISCORD_TOKEN";

#[tokio::main]
async fn main() -> Result<()> {
    let handler = DiscordHandle::new()?;
    handler.setup(ReginaldConfig::get_conf()?)?;

    let token = env::var(DISCORD_TOKEN_ENV)
        .with_context(|| format!("Missing environment variable: {DISCORD_TOKEN_ENV}"))?;

    run_bot(handler, token).await
}
