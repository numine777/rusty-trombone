use anyhow::anyhow;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            CommandResult,
        },
        StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::GatewayIntents,
};
use shuttle_secrets::SecretStore;
use songbird::SerenityInit;
use tracing::info;

mod commands;
use crate::commands::{channel::*, play::*, queue::*, utils::*, volume::*};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "Pong!").await);
    return Ok(());
}

#[group]
#[commands(deafen, join, leave, mute, play, ping, undeafen, unmute, skip, reset)]
struct General;

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("r!"))
        .group(&GENERAL_GROUP);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    Ok(client)
}
