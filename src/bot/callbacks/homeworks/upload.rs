use crate::types::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage};

pub async fn upload(
    bot: Bot,
    msg: MaybeInaccessibleMessage,
    page: u64,
    hw_type: u8,
    group_id: u64,
    upload_hw_id: u64,
    evaluate: Option<u64>,
) -> HandlerResult {
    let Some(_evaluate) = evaluate else {
        return Ok(());
    };

    let cb = |rating: u8| {
        format!("homeworks/{group_id}/{hw_type}/{page}/upload/{upload_hw_id}/{rating}")
    };
    
    let kb = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("⭐", cb(1)),
            InlineKeyboardButton::callback("⭐⭐⭐", cb(3)),
        ],
        vec![
            InlineKeyboardButton::callback("⭐⭐", cb(2)),
            InlineKeyboardButton::callback("⭐⭐⭐⭐", cb(4)),
        ],
        vec![InlineKeyboardButton::callback("⭐⭐⭐⭐⭐", cb(5))],
        vec![InlineKeyboardButton::callback(
            "Назад",
            format!("homeworks/{group_id}/{hw_type}/{page}/upload"),
        )],
    ]);

    bot.edit_message_text(
        msg.chat().id,
        msg.id(),
        "Оцените, насколько знания, полученные на уроке, были достаточными для выполнения ДЗ:",
    )
    .reply_markup(kb)
    .await?;

    Ok(())
}
