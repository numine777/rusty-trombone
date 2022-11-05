use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::client::Context;
use serenity::model::channel::Message;

use super::utils::check_msg;

#[command]
#[only_in(guilds)]
pub async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let manager = songbird::get(ctx).await
        .expect("Songbird voice client passed in at initialization").clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        check_msg(msg.channel_id.say(&ctx.http, "Already deafened").await);
    } else {
        if let Err(e) = handler.deafen(true).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Deafened").await);
    }

    return Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn undeafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let manager = songbird::get(ctx).await
        .expect("Songbird voice client passed in at initialization").clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        if let Err(e) = handler.deafen(false).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "undeafened").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Already undeafened").await);
    }

    return Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    return Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    return Ok(())
}

