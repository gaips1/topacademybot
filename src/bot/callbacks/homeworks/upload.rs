use crate::{bot::keyboards::{cancel_kb, mm_kb}, types::*};
use teloxide::{
    net::Download,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage},
};

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

pub async fn receive_text_answer(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    (homework_id, evaluate, hours, minutes): (u64, u8, u8, u8),
) -> HandlerResult {
    let Some(text) = msg.text() else {
        bot.send_message(msg.chat.id, "Отправьте ваш текстовый ответ (или <code>пропустить</code>, чтобы пропустить этот этап):")
            .reply_markup(cancel_kb())
            .parse_mode(teloxide::types::ParseMode::Html)
            .await?;
        return Ok(());
    };
    let text = text.trim();

    if text.len() < 5 {
        bot.send_message(msg.chat.id, "Минимум 5 символов. Отправьте ваш текстовый ответ (или <code>пропустить</code>, чтобы пропустить этот этап):")
            .reply_markup(cancel_kb())
            .parse_mode(teloxide::types::ParseMode::Html)
            .await?;
        return Ok(());
    }

    let text_answer = if text.to_lowercase() == "пропустить" {
        None
    } else {
        Some(text.to_string())
    };

    let is_some = text_answer.is_some();

    dialogue
        .update(State::HomeworkReceiveFileAnswer {
            homework_id,
            evaluate,
            time_hours: hours,
            time_minutes: minutes,
            text_answer,
        })
        .await?;

    bot.send_message(
        msg.chat.id,
        format!(
            "Отправьте файл с выполненной домашней работой.\n\
            Рекомендуется загружать файлы в архиве. Файлы в формате .тхт и .csv не допускаются.{}",
            if is_some {
                "\n\n(или <code>пропустить</code>, чтобы пропустить этот этап)"
            } else {
                ""
            }
        ),
    )
    .reply_markup(cancel_kb())
    .parse_mode(teloxide::types::ParseMode::Html)
    .await?;

    Ok(())
}

pub async fn receive_file_answer(
    bot: Bot,
    user: User,
    dialogue: MyDialogue,
    msg: Message,
    (homework_id, evaluate, hours, minutes, text_answer): (u64, u8, u8, u8, Option<String>),
) -> HandlerResult {
    let text = msg.text().map(|t| t.trim().to_lowercase());
    if text.as_deref() == Some("пропустить") {
        if text_answer.is_none() {
            bot.send_message(
                msg.chat.id,
                "Отправьте файл с выполненной домашней работой:",
            )
            .reply_markup(cancel_kb())
            .await?;
            return Ok(());
        }

        let result = user
            .create_homework(
                homework_id,
                None,
                None,
                hours,
                minutes,
                evaluate,
                text_answer,
            )
            .await;

        dialogue.exit().await?;

        let reply = match result {
            Ok(_) => "Домашнее задание успешно отправлено на проверку!",
            Err(err) => err.user_message(),
        };
        bot.send_message(msg.chat.id, reply)
            .reply_markup(mm_kb())
            .await?;
        
        return Ok(());
    }

    let now = chrono::Local::now().format("%d-%m-%Y_%H-%M-%S");

    let (file_id, file_name): (teloxide::types::FileId, String) = if let Some(doc) = msg.document()
    {
        (doc.file.id.clone(), doc.file_name.clone().unwrap())
    } else if let Some(photos) = msg.photo() {
        let photo = photos.last().unwrap();
        (photo.file.id.clone(), format!("photo-{}.jpg", now))
    } else if let Some(video) = msg.video() {
        (
            video.file.id.clone(),
            video
                .file_name
                .clone()
                .unwrap_or_else(|| format!("video-{}.mp4", now)),
        )
    } else {
        bot.send_message(msg.chat.id, "Пожалуйста, отправьте файл, фото или видео:")
            .reply_markup(cancel_kb())
            .await?;
        return Ok(());
    };

    let file = bot.get_file(file_id).await?;
    let mut buf: Vec<u8> = Vec::new();
    bot.download_file(&file.path, &mut buf).await?;

    let result = user
        .create_homework(
            homework_id,
            Some(buf),
            Some(file_name),
            hours,
            minutes,
            evaluate,
            text_answer,
        )
        .await;

    dialogue.exit().await?;

    let reply = match result {
        Ok(_) => "Домашнее задание успешно отправлено на проверку!",
        Err(err) => err.user_message(),
    };
    bot.send_message(msg.chat.id, reply)
        .reply_markup(mm_kb())
        .await?;

    Ok(())
}
