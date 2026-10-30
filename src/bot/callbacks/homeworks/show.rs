use std::fmt::Write as _;
use crate::{bot::{ext::DateTimeFormatExt, helpers::send_split}, types::*};
use chrono::NaiveDateTime;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage};

const CATEGORIES: [(u8, &str); 4] = [
    (3, "Текущие дз"),
    (2, "На проверке"),
    (1, "Проверено"),
    (0, "Просрочено"),
];

fn kb(page: usize, hw_type: u8) -> InlineKeyboardMarkup {
    let mut nav = Vec::with_capacity(2);
    if page > 1 {
        nav.push(InlineKeyboardButton::callback(
            "Назад",
            format!("homeworks/{}/{}", hw_type, page - 1),
        ));
    }
    nav.push(InlineKeyboardButton::callback(
        "Вперёд",
        format!("homeworks/{}/{}", hw_type, page + 1),
    ));

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
            format!("homeworks/{id}/1"),
        )]);
    }

    rows.push(vec![InlineKeyboardButton::callback("В главное меню", "mm")]);

    InlineKeyboardMarkup::new(rows)
}

pub async fn show(
    bot: Bot,
    msg: MaybeInaccessibleMessage,
    homeworks: HomeworkList,
    page: usize,
    hw_type: u8,
) -> HandlerResult {
    if homeworks.is_empty() {
        bot.edit_message_text(
            msg.chat().id,
            msg.id(),
            "В данный момент выбранных домашних заданий нет.",
        )
        .reply_markup(kb(page, hw_type))
        .await?;

        return Ok(());
    }

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

    send_split(&bot, msg.chat().id, msg.id(), &text, kb(page, hw_type)).await?;
    Ok(())
}
