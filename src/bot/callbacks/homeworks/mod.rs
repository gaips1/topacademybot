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

    let path = format!(
        "/homework/operations/list?page={}&status={}&type=0&group_id={}",
        page, hw_type, user_data.current_group_id
    );

    let homeworks = match user.request_path::<HomeworkList>(&path).await {
        Ok(data) => data,
        Err(err) => {
            let _ = bot.send_message(msg.chat().id, err.user_message()).await;
            return Ok(());
        }
    };

    let path = format!(
        "/count/homework?type=0&group_id={}",
        user_data.current_group_id
    );

    let counters = match user.request_path::<HomeworkCounterList>(&path).await {
        Ok(data) => data,
        Err(err) => {
            let _ = bot.send_message(msg.chat().id, err.user_message()).await;
            return Ok(());
        }
    };

    bot.answer_callback_query(q.id).await?;
    show::show(bot, msg, counters, homeworks, page, hw_type).await?;

    Ok(())
}
