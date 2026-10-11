use anyhow::Context;
use teloxide::{dispatching::dialogue::GetChatId, prelude::*};

pub async fn cancel_callback_handler(
    bot: Bot,
    q: CallbackQuery,
    dialogue: super::types::MyDialogue,
) -> super::types::HandlerResult {
    let chat_id = q.chat_id().context("Не в чате")?;
    let message_id = q.message.context("Нет сообщения")?.id();

    bot.delete_message(chat_id, message_id).await?;
    dialogue.exit().await?;
    let _ = bot.answer_callback_query(q.id).text("Отменено!").await;

    Ok(())
}
