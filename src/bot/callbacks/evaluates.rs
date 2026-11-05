use std::time::Duration;

use crate::{ext::DateTimeFormatExt, try_or_reply, types::*};
use chrono::NaiveDate;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use tokio::time::sleep;

fn kb(key: &str, eval_1: Option<u8>) -> InlineKeyboardMarkup {
    let btn = |n: u8| {
        let data = match eval_1 {
            Some(e) => format!("evaluate/{key}/{e}/{n}"),
            None => format!("evaluate/{key}/{n}"),
        };
        InlineKeyboardButton::callback("⭐".repeat(n as usize), data)
    };

    InlineKeyboardMarkup::new([vec![btn(1), btn(3)], vec![btn(2), btn(4)], vec![btn(5)]])
}

pub async fn send_evaluate(bot: Bot, chat_id: ChatId, evaluate: &Evaluate) -> HandlerResult {
    bot.send_message(
        chat_id,
        format!(
            "Оцените занятие: {} {}\n\n\
            Оцените работу преподавателя {}:",
            evaluate.spec_name.to_uppercase(),
            NaiveDate::from_rus(&evaluate.date_visit).to_rus(),
            evaluate.fio_teach
        ),
    )
    .reply_markup(kb(&evaluate.key, None))
    .await?;

    Ok(())
}

pub async fn evaluate_callback_handler(
    bot: Bot,
    q: CallbackQuery,
    user: User,
) -> super::types::HandlerResult {
    let msg = q.regular_message().context("Нет сообщения")?;
    let mut data = q.data.as_deref().unwrap().split('/').skip(1);

    let key = data.next().context("нет ключа")?;
    let eval_1: Option<u8> = data.next().and_then(|e| Some(e.parse().ok()?));
    let eval_2: Option<u8> = data.next().and_then(|e| Some(e.parse().ok()?));

    if eval_1.is_some() && eval_2.is_some() {
        let evaluations = try_or_reply!(
            user.evaluate_lesson(q.from.id, key, eval_1.unwrap(), eval_2.unwrap()),
            bot,
            msg.chat.id
        );

        bot.edit_message_text(msg.chat.id, msg.id, "Спасибо за Ваши оценки!")
            .await?;

        sleep(Duration::from_millis(1_500)).await;

        if let Some(first) = evaluations.first() {
            send_evaluate(bot, msg.chat.id, first).await?;
        }

        return Ok(());
    }

    let header = msg
        .text()
        .context("нет текста")?
        .split('\n')
        .next()
        .unwrap();

    bot.edit_message_text(
        msg.chat.id,
        msg.id,
        format!("{header}\n\nТеперь оцените качество урока:"),
    )
    .reply_markup(kb(key, eval_1))
    .await?;

    Ok(())
}
