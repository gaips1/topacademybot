use crate::bot::types;
use teloxide::prelude::*;

mod cancel;

pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    dialogue: types::MyDialogue,
) -> types::HandlerResult {
    if let Some(data) = q.data.as_deref() {
        match data {
            "cancel" => cancel::cancel_callback_handler(bot, q, dialogue).await?,
            _ => {}
        };
    }
    Ok(())
}
