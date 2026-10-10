use anyhow::Context;
use teloxide::{dispatching::dialogue::GetChatId, prelude::*};

pub async fn activity_callback_handlers(
    bot: Bot,
    q: CallbackQuery
) -> super::types::HandlerResult {
    let chat_id = q.chat_id().context("Не в чате")?;
    let message_id = q.message.context("Нет сообщения")?.id();

    todo!();

    Ok(())
}
