use crate::{
    bot::{ext::DateTimeFormatExt, helpers::send_split},
    types::*,
};
use chrono::NaiveDateTime;
use std::fmt::Write as _;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage};

const CATEGORIES: [(u8, &str); 4] = [
    (3, "Текущие дз"),
    (2, "На проверке"),
    (1, "Проверено"),
    (0, "Просрочено"),
];

fn kb(page: i64, hw_type: u8, group_id: u64, counter: i64) -> InlineKeyboardMarkup {
    let mut nav = Vec::with_capacity(2);
    if page > 1 {
        nav.push(InlineKeyboardButton::callback(
            "Назад",
            format!("homeworks/{group_id}/{hw_type}/{}", page - 1),
        ));
    }

    if counter - (page * 6) > 0 {
        nav.push(InlineKeyboardButton::callback(
            "Вперёд",
            format!("homeworks/{group_id}/{hw_type}/{}", page + 1),
        ));
    }

    let mut rows = vec![nav];

    for (id, name) in CATEGORIES {
        rows.push(vec![InlineKeyboardButton::callback(
            format!(
                "{name}{}",
                if hw_type == id {
                    " (Вы здесь)"
                } else {
                    ""
                }
            ),
            format!("homeworks/{group_id}/{id}/1"),
        )]);
    }

    rows.push(vec![InlineKeyboardButton::callback("В главное меню", "mm")]);

    InlineKeyboardMarkup::new(rows)
}

pub async fn show(
    bot: Bot,
    msg: MaybeInaccessibleMessage,
    user: User,
    page: u64,
    hw_type: u8,
    group_id: u64,
) -> HandlerResult {
    let path = format!("/count/homework?type=0&group_id={group_id}");
    let counters = match user.request_path::<HomeworkCounterList>(&path).await {
        Ok(data) => data,
        Err(err) => {
            let _ = bot.send_message(msg.chat().id, err.user_message()).await;
            return Ok(());
        }
    };

    let counter = counters.iter().find(|c| c.counter_type == hw_type);
    let Some(counter) = counter else {
        bot.edit_message_text(
            msg.chat().id,
            msg.id(),
            "В данный момент выбранных домашних заданий нет.",
        )
        .reply_markup(kb(page as i64, hw_type, group_id, 0))
        .await?;

        return Ok(());
    };

    let path = format!(
        "/homework/operations/list?page={page}&status={hw_type}&type=0&group_id={group_id}",
    );
    let homeworks = match user.request_path::<HomeworkList>(&path).await {
        Ok(data) => data,
        Err(err) => {
            let _ = bot.send_message(msg.chat().id, err.user_message()).await;
            return Ok(());
        }
    };

    let mut text = String::new();
    for hw in homeworks {
        let creation_time = NaiveDateTime::from_rus(&hw.creation_time);
        let comletion_time = NaiveDateTime::from_rus(&hw.completion_time);

        let theme: String = hw.theme.chars().take(50).collect();
        let ellipsis = if hw.theme.chars().count() > 50 {
            "..."
        } else {
            ""
        };

        let _ = write!(
            &mut text,
            concat!(
                "📚 <b>{}</b>\n",
                "    📌 <b>Тема:</b> {}{}\n",
                "    👨‍🏫 <b>Преподаватель:</b> {}\n",
                "    📅 <b>Выдано:</b> {}\n",
                "    📅 <b>Сдать до:</b> {}\n",
                "    ℹ️ <b>Комментарий:</b> {}\n",
                "    📩 <a href=\"{}\">Скачать назначенное дз</a>\n\n"
            ),
            hw.name_spec,
            theme,
            ellipsis,
            hw.fio_teach,
            creation_time.to_rus_date(),
            comletion_time.to_rus_date(),
            hw.comment,
            hw.file_path
        );
    }

    send_split(
        &bot,
        msg.chat().id,
        msg.id(),
        &text,
        kb(page as i64, hw_type, group_id, counter.counter as i64),
    )
    .await?;
    Ok(())
}
