use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::client::Context;
use serenity::model::channel::Message;

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    return Ok(())
}
