use crate::{bot::{ext::DateTimeFormatExt, helpers::send_split_text}, types::*};
use anyhow::Context;
use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime};
use std::fmt::Write;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

pub async fn activity_callback_handlers(
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

    let data = match user.request::<ActivityList>().await {
        Ok(data) => data,
        Err(err) => {
            let _ = bot
                .edit_message_text(msg.chat().id, msg.id(), err.user_message())
                .await;
            return Ok(());
        }
    };

    let today = chrono::Local::now().date_naive();
    let monday = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let start = (monday - Duration::days(page as i64 * 7)).and_time(NaiveTime::MIN);
    let end = start + Duration::days(5);

    let mut week: Vec<_> = data
        .iter()
        .filter(|a| {
            let d = NaiveDateTime::from_rus(&a.date_visit);
            d >= start && d < end
        })
        .collect();

    week.sort_by_key(|a| (NaiveDateTime::from_rus(&a.date_visit), a.lesson_number));
    week.dedup_by_key(|a| (NaiveDateTime::from_rus(&a.date_visit), a.lesson_number, a.spec_name.clone()));

    let mut text = format!(
        "<b>🗓️ Учебная неделя {} - {}</b>\n\n",
        start.to_rus_date(),
        (end - Duration::days(1)).to_rus_date()
    );

    for a in week {
        let marks: Vec<String> = vec![
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
            "🎓 <b>{}</b>\n\
            - <b>Пара:</b> {}\n\
            - <b>Посещение:</b> {}\n\
            - <b>Преподаватель:</b> {}\n\
            - <b>Тема:</b> {}{}\n\
            - <b>Оценки:</b> {}\n\n",
            a.spec_name,
            a.lesson_number.unwrap_or(0),
            a.status_was(),
            a.teacher_name,
            theme,
            ellipsis,
            if marks.len() == 0 { "Нет".to_string() } else { marks.join(", ") }
        );
    }

    let _ = bot.answer_callback_query(q.id).await;

    send_split_text(bot, msg.chat().id, msg.id(), &text).await?;

    Ok(())
}
