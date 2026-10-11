use crate::types::*;
use anyhow::Context;
use chrono::NaiveDateTime;
use std::fmt::Write;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

fn kb(page: usize) -> InlineKeyboardMarkup {
    let mut nav = Vec::with_capacity(2);
    if page > 0 {
        nav.push(InlineKeyboardButton::callback(
            "Назад",
            format!("rewards/{}", page - 1),
        ));
    }
    nav.push(InlineKeyboardButton::callback(
        "Вперёд",
        format!("rewards/{}", page + 1),
    ));

    InlineKeyboardMarkup::new([
        nav,
        vec![InlineKeyboardButton::callback("В главное меню", "mm")],
    ])
}

pub async fn rewards_callback_handlers(
    bot: Bot,
    q: CallbackQuery,
    user: User,
) -> super::types::HandlerResult {
    let msg = q.message.context("Нет сообщения")?;
    let page: usize = q
        .data
        .unwrap()
        .split('/')
        .nth(1)
        .context("нет страницы")?
        .parse()?;

    let data = match user.request::<RewardsHistory>().await {
        Ok(data) => data,
        Err(err) => {
            let _ = bot
                .edit_message_text(msg.chat().id, msg.id(), err.user_message())
                .await;
            return Ok(());
        }
    };

    let mut text = "<b>🏆 Ваши награды 🏆</b>\n\n".to_string();
    for r in data.iter().skip(page * 10).take(10) {
        let date = NaiveDateTime::parse_from_str(&r.date, "%Y-%m-%d %H:%M:%S")
            .unwrap_or_else(|_| chrono::Local::now().naive_local());

        write!(
            &mut text,
            "<b>{}</b>\n{}: {}{} {}\n\
            ---------------------------------------\n",
            date.format("%d.%m.%Y %H:%M"),
            r.achievements_name.translation(),
            if r.action == 1 { '+' } else { '-' },
            r.current_point,
            r.point_types_name.translation()
        )?
    }

    let _ = bot.answer_callback_query(q.id).await;

    bot.edit_message_text(msg.chat().id, msg.id(), text)
        .parse_mode(teloxide::types::ParseMode::Html)
        .reply_markup(kb(page))
        .await?;

    Ok(())
}
