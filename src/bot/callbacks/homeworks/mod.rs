use crate::types::*;

mod show;
mod upload;

pub async fn homeworks_callbacks_handler(
    bot: Bot,
    q: CallbackQuery,
    user: User,
) -> super::types::HandlerResult {
    let msg = q.message.context("Нет сообщения")?;
    let mut data = q.data.as_deref().unwrap().split('/').skip(1);
    let group_id: u64 = data.next().context("нет айди группы")?.parse()?;
    let hw_type: u8 = data.next().context("нет типа")?.parse()?;
    let page: u64 = data.next().context("нет страницы")?.parse()?;
    let _is_upload = data.next() == Some("upload");

    if let Some(_upload_hw_id) = data.next() {
        return Ok(());
    }

    bot.answer_callback_query(q.id).await?;
    show::show(bot, msg, user, page, hw_type, group_id).await?;

    Ok(())
}
