use crate::{bot::keyboards::cancel_kb, types::*};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage};

pub async fn upload(
    bot: Bot,
    msg: MaybeInaccessibleMessage,
    page: u64,
    hw_type: u8,
    group_id: u64,
    upload_hw_id: u64,
    evaluate: Option<u8>,
    dialogue: MyDialogue,
) -> HandlerResult {
    if let Some(evaluate) = evaluate {
        dialogue
            .update(State::HomeworkReceiveTime {
                homework_id: upload_hw_id,
                evaluate,
            })
            .await?;

        bot.edit_message_text(
            msg.chat().id,
            msg.id(),
            "Потрачено времени на выполнение ДЗ (чч:мм):",
        )
        .reply_markup(cancel_kb())
        .await?;

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

pub async fn receive_time(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    (homework_id, evaluate): (u64, u8),
) -> HandlerResult {
    let text = msg.text().unwrap_or_default().trim();

    let Some((h, m)) = text.split_once(':') else {
        bot.send_message(msg.chat.id, "Формат: чч:мм (например 1:45, 09:05):")
            .reply_markup(cancel_kb())
            .await?;
        return Ok(());
    };

    // ебучий бойлерплейт
    // пусть так останется)
    let Ok(hours) = h.trim().parse::<u8>() else {
        bot.send_message(msg.chat.id, "Часы 0–23, минуты 0–59:")
            .reply_markup(cancel_kb())
            .await?;
        return Ok(());
    };

    let Ok(minutes) = m.trim().parse::<u8>() else {
        bot.send_message(msg.chat.id, "Часы 0–23, минуты 0–59:")
            .reply_markup(cancel_kb())
            .await?;
        return Ok(());
    };

    if hours > 23 || minutes > 59 {
        bot.send_message(msg.chat.id, "Часы 0–23, минуты 0–59:")
            .reply_markup(cancel_kb())
            .await?;
        return Ok(());
    }

    dialogue
        .update(State::HomeworkReceiveTextAnswer {
            homework_id,
            evaluate,
            time_hours: hours,
            time_minutes: minutes,
        })
        .await?;

    bot.send_message(
        msg.chat.id,
        "Отправьте ваш текстовый ответ (или <code>пропустить</code>, чтобы пропустить этот этап):",
    )
    .reply_markup(cancel_kb())
    .parse_mode(teloxide::types::ParseMode::Html)
    .await?;

    Ok(())
}
