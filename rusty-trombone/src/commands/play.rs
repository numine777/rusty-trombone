use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::client::Context;
use serenity::model::channel::Message;

use crate::commands::utils::check_msg;

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.reply(ctx, "Play pressed").await);
    return Ok(())
}
