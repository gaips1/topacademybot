use crate::bot::types;
use teloxide::prelude::*;

mod cancel;
mod activity;

pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    dialogue: types::MyDialogue,
) -> types::HandlerResult {
    let custom_id = q.data.as_deref().unwrap_or("");
    let prefix = custom_id.split('/').next().unwrap_or(custom_id);

    match prefix {
        "cancel" => cancel::cancel_callback_handler(bot, q, dialogue).await?,
        "activity" => activity::activity_callback_handlers(bot, q).await?,
        _ => {}
    }

    Ok(())
}
