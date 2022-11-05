use serenity::{model::channel::Message, Result as SerenityResult};
use tracing::error;

pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(result) = result {
        error!("Error sending message: {:?}", result);
    }
}
