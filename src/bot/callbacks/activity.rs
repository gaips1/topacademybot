use crate::{
    bot::{ext::DateTimeFormatExt, helpers::send_split},
    types::*,
};
use anyhow::Context;
use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime};
use std::fmt::Write;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

fn kb(page: usize) -> InlineKeyboardMarkup {
    let mut nav = Vec::with_capacity(2);
    if page > 0 {
        nav.push(InlineKeyboardButton::callback(
            "Назад",
            format!("activity/{}", page - 1),
        ));
    }
    nav.push(InlineKeyboardButton::callback(
        "Вперёд",
        format!("activity/{}", page + 1),
    ));

    InlineKeyboardMarkup::new([
        nav,
        vec![InlineKeyboardButton::callback("В главное меню", "mm")],
    ])
}

pub async fn activity_callback_handler(
    bot: Bot,
    q: CallbackQuery,
    user: User,
) -> super::types::HandlerResult {
    let msg = q.message.context("Нет сообщения")?;
    let page: usize = q.data.unwrap().split('/').nth(1).unwrap_or("0").parse()?;

    let data = match user.request::<ActivityList>().await {
        Ok(data) => data,
        Err(err) => {
            let _ = bot.send_message(msg.chat().id, err.user_message()).await;
            return Ok(());
        }
    };

    let today = chrono::Local::now().date_naive();
    let monday = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let start = (monday - Duration::days(page as i64 * 7)).and_time(NaiveTime::MIN);
    let end = start + Duration::days(5);

    let mut week: Vec<_> = data
        .iter()
        .filter_map(|a| {
            let d = NaiveDateTime::from_rus(&a.date_visit);
            if d >= start && d < end {
                Some((d, a))
            } else {
                None
            }
        })
        .collect();

    week.sort_by_key(|a| (a.0, a.1.lesson_number));

    let mut text = format!(
        "<b>🗓️ Учебная неделя {} - {}</b>\n\n",
        start.to_rus_date(),
        (end - Duration::days(1)).to_rus_date()
    );

    for activities in week.chunk_by(|a, b| a.0.date() == b.0.date()) {
        if let Some((first, _)) = activities.first() {
            let _ = write!(&mut text, "<b><u>{}</u></b>\n", first.to_rus_date());
        } else {
            let _ = bot.answer_callback_query(q.id).await;

            bot.edit_message_text(msg.chat().id, msg.id(), "Вы не учились на этой неделе")
                .reply_markup(kb(page))
                .await?;

            return Ok(());
        }

        for (_, a) in activities {
            let marks: Vec<String> = [
                a.control_work_mark,
                a.home_work_mark,
                a.lab_work_mark,
                a.class_work_mark,
                a.practical_work_mark,
            ]
            .into_iter()
            .filter_map(|m| Some(m?.to_string()))
            .collect();

            let theme: String = a.lesson_theme.chars().take(50).collect();
            let ellipsis = if a.lesson_theme.chars().count() > 50 {
                "..."
            } else {
                ""
            };

            let _ = write!(
                &mut text,
                concat!(
                    "🎓 <b>{}</b>\n",
                    "\u{2800}\u{2800}- <b>Пара:</b> {}\n",
                    "\u{2800}\u{2800}- <b>Посещение:</b> {}\n",
                    "\u{2800}\u{2800}- <b>Преподаватель:</b> {}\n",
                    "\u{2800}\u{2800}- <b>Тема:</b> {}{}\n",
                    "\u{2800}\u{2800}- <b>Оценки:</b> {}\n\n",
                ),
                a.spec_name,
                a.lesson_number.unwrap_or(0),
                a.status_was(),
                a.teacher_name,
                theme,
                ellipsis,
                if marks.is_empty() {
                    "Нет".to_string()
                } else {
                    marks.join(", ")
                }
            );
        }
    }

    let _ = bot.answer_callback_query(q.id).await;

    send_split(&bot, msg.chat().id, msg.id(), &text, kb(page)).await?;
    Ok(())
}
