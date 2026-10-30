use crate::types::*;
use anyhow::Context;
use std::fmt::Write;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

fn kb(is_group: bool) -> InlineKeyboardMarkup {
    let mut nav = Vec::with_capacity(1);

    if is_group {
        nav.push(InlineKeyboardButton::callback(
            "Таблица лидеров (в потоке)",
            "leaderboard/stream",
        ));
    } else {
        nav.push(InlineKeyboardButton::callback(
            "Таблица лидеров (в группе)",
            "leaderboard/group",
        ));
    }

    InlineKeyboardMarkup::new([
        nav,
        vec![InlineKeyboardButton::callback("В главное меню", "mm")],
    ])
}

pub async fn leaderboard_callback_handler(
    bot: Bot,
    q: CallbackQuery,
    user: User,
) -> super::types::HandlerResult {
    let msg = q.message.context("Нет сообщения")?;
    let is_group = q.data.as_deref() == Some("leaderboard/group");

    let result = if is_group {
        user.request::<StudentRatingListGroup>().await.map(|d| d.0)
    } else {
        user.request::<StudentRatingListStream>().await.map(|d| d.0)
    };

    let data: Vec<StudentRating> = match result {
        Ok(data) => data,
        Err(err) => {
            let _ = bot
                .edit_message_text(msg.chat().id, msg.id(), err.user_message())
                .await;
            return Ok(());
        }
    };

    let mut text = "<b>🏆 Таблица лидеров 🏆</b>\n\n".to_string();
    for l in data.iter().filter(|r| r.full_name.is_some()) {
        write!(
            &mut text,
            "{}. {} - {} топмани\n",
            l.position,
            l.full_name.as_ref().unwrap(),
            l.amount
        )?
    }

    let _ = bot.answer_callback_query(q.id).await;

    bot.edit_message_text(msg.chat().id, msg.id(), text)
        .parse_mode(teloxide::types::ParseMode::Html)
        .reply_markup(kb(is_group))
        .await?;

    Ok(())
}
