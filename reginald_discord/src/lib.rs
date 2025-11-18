use anyhow::Result;
use reginald_ast::ast::{
    base::{env::ReginaldEnv, money::Denomination},
    eval::evaluate,
    typechecker::typed_ast::Value,
};
use reginald_conf::ReginaldConfig;
use serenity::{
    Client,
    all::{Context, EventHandler, GatewayIntents},
    async_trait,
    model::{channel::Message, gateway::Ready},
};
use sqlite::{Connection, ConnectionThreadSafe, State};
use std::str::FromStr;

pub struct DiscordHandle {
    conn: ConnectionThreadSafe,
}

const SETUP_QUERIES: &str = include_str!("../queries/db_setup.sql");
const INSERT_PLAYER_QUERY: &str = include_str!("../queries/db_insert_character.sql");
const GET_ENV_QUERY: &str = include_str!("../queries/get_env.sql");
const UPDATE_ENV_QUERY: &str = include_str!("../queries/update_env.sql");

impl DiscordHandle {
    pub fn new() -> Result<Self> {
        let conn = Connection::open_thread_safe("reginald_db")?;

        Ok(Self { conn })
    }

    pub fn setup(&self, conf: ReginaldConfig) -> Result<()> {
        self.conn.execute(SETUP_QUERIES)?;
        for plr in conf.players {
            let mut stmt = self.conn.prepare(INSERT_PLAYER_QUERY)?;
            stmt.bind(&[(":name", plr.name.as_str()), (":class", plr.class.as_str())][..])?;
            stmt.next()?;
        }

        let mut stmt = self.conn.prepare(INSERT_PLAYER_QUERY)?;
        stmt.bind(&[(":name", conf.dm.as_str()), (":class", "dm")][..])?;
        stmt.next()?;
        Ok(())
    }

    pub fn get_env(&self) -> Result<ReginaldEnv> {
        let mut stmt = self.conn.prepare(GET_ENV_QUERY)?;
        let mut env = ReginaldEnv::default();
        while let Ok(State::Row) = stmt.next() {
            let denomination = stmt.read::<String, _>("item")?;
            let count = stmt.read::<i64, _>("count")?;
            env = env.checked_add(Value::Piece(count, Denomination::from_str(&denomination)?))?;
        }

        Ok(env)
    }

    pub fn update_env(&self, env: ReginaldEnv) -> Result<()> {
        let mut stmt = self.conn.prepare(UPDATE_ENV_QUERY)?;
        stmt.bind(
            &[
                (":pp", env.pp),
                (":ep", env.ep),
                (":gp", env.gp),
                (":sp", env.sp),
                (":cp", env.cp),
            ][..],
        )?;
        while let Ok(res) = stmt.next() {
            if res == State::Done {
                break;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl EventHandler for DiscordHandle {
    // Set a handler for the `message` event. This is called whenever a new message is received.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be
    // dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        let env = self.get_env();
        if let Err(err) = &env {
            let output = format!("{:?}", err);
            if let Err(msg_err) = msg.channel_id.say(&ctx.http, &output).await {
                eprintln!("Error: {msg_err:?}, sending: {output}");
            }
            return;
        }
        let env = env.unwrap();
        match evaluate(&msg.content, env) {
            Ok(env) => {
                let output = env.output().join(" ");
                if let Err(msg_err) = msg.channel_id.say(&ctx.http, &output).await {
                    eprintln!("Error: {msg_err:?}, sending: {output}");
                }
            }
            Err(reg_err) => {
                let output = format!("{:?}", &reg_err);
                if let Err(msg_err) = msg.channel_id.say(&ctx.http, &output).await {
                    eprintln!("Error: {msg_err:?}, sending: {output}");
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn run_bot(handler: DiscordHandle, token: String) -> Result<()> {
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await?;

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    client.start().await?;

    Ok(())
}
