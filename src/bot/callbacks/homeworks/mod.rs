use crate::{
    api::{BASE_URL, REQWEST_CLIENT},
    types::*,
};

mod show;
mod upload;

pub async fn homeworks_callbacks_handler(
    bot: Bot,
    q: CallbackQuery,
    user: User,
) -> super::types::HandlerResult {
    let msg = q.message.context("Нет сообщения")?;
    let mut data = q.data.as_deref().unwrap().split('/').skip(1);
    let hw_type: u8 = data.next().context("нет типа")?.parse()?;
    let page: usize = data.next().context("нет страницы")?.parse()?;
    let _is_upload = data.next() == Some("upload");

    if let Some(_upload_hw_id) = data.next() {
        return Ok(());
    }

    let user_data = match user.request::<UserData>().await {
        Ok(data) => data,
        Err(err) => {
            let _ = bot.send_message(msg.chat().id, err.user_message()).await;
            return Ok(());
        }
    };

    let builder = REQWEST_CLIENT
        .get(format!(
            "{BASE_URL}/homework/operations/list?page={}&status={}&type=0&group_id={}",
            page, hw_type, user_data.current_group_id
        ))
        .header("Accept", "application/json, text/plain, */*")
        .header("Referer", "https://journal.top-academy.ru/")
        .header("Content-Type", "application/json");

    let homeworks = match user.request_with_options::<HomeworkList>(builder).await {
        Ok(data) => data,
        Err(err) => {
            let _ = bot.send_message(msg.chat().id, err.user_message()).await;
            return Ok(());
        }
    };

    bot.answer_callback_query(q.id).await?;
    show::show(bot, msg, homeworks, page, hw_type).await?;

    Ok(())
}
