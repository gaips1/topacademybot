use crate::bot::types;
use anyhow::Context;
use teloxide::prelude::*;

mod activity;
pub mod cancel;
pub mod homeworks;
mod leaderboard;
mod rewards;

pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    dialogue: types::MyDialogue,
    user: crate::types::User,
) -> types::HandlerResult {
    let custom_id = q.data.as_deref().unwrap_or("");
    let prefix = custom_id.split('/').next().unwrap_or(custom_id);
    let msg = q.message.as_ref().context("нет сообщения")?;

    match prefix {
        "cancel" => cancel::cancel_callback_handler(bot, q, dialogue).await?,
        "rewards" => rewards::rewards_callback_handlers(bot, q, user).await?,
        "leaderboard" => leaderboard::leaderboard_callback_handler(bot, q, user).await?,
        "activity" => activity::activity_callback_handler(bot, q, user).await?,
        "homeworks" => homeworks::homeworks_callbacks_handler(bot, q, user, dialogue).await?,
        "mm" => {
            crate::bot::commands::start_command(bot, msg.chat().id, msg.id(), user, true).await?
        }
        _ => {}
    }

    Ok(())
}
