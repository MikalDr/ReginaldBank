use anyhow::Result;
use reginald_ast::ast::{base::env::ReginaldEnv, eval::evaluate};
use reginald_db::DBHandle;
use serenity::{
    Client,
    all::{Context, EventHandler, GatewayIntents},
    async_trait,
    model::{channel::Message, gateway::Ready},
};

pub struct DiscordHandle {
    db: DBHandle,
}

impl DiscordHandle {
    pub fn new() -> Result<Self> {
        Ok(Self {
            db: DBHandle::new()?,
        })
    }

    pub fn get_env(&self) -> Result<ReginaldEnv> {
        self.db.get_env()
    }

    pub fn update_env(&self, env: ReginaldEnv) -> Result<()> {
        self.db.update_env(env)
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
