use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::client::Context;
use serenity::model::channel::Message;

use crate::commands::utils::check_msg;

#[command]
#[only_in(guilds)]
pub async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.reply(ctx, "Skip pressed").await);
    return Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn reset(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.reply(ctx, "Reset pressed").await);
    return Ok(())
}
