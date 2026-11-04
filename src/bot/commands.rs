use std::time::Duration;

use crate::{
    bot::callbacks::evaluates::send_evaluate, ext::DateTimeFormatExt, try_or_reply, types::*,
};
use chrono::{NaiveDate, NaiveDateTime};
use teloxide::{macros::BotCommands, prelude::*, types::MessageId};
use tokio::time::sleep;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
    Relogin,
}

pub async fn on_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    user: User,
    dialogue: super::types::MyDialogue,
) -> super::types::HandlerResult {
    match cmd {
        Command::Start => {
            start_command(
                bot,
                msg.chat.id,
                msg.id,
                msg.from.context("нет юзера")?.id,
                user,
                false,
            )
            .await?
        }
        Command::Relogin => relogin_command(bot, msg, dialogue).await?,
    }
    Ok(())
}

pub async fn start_command(
    bot: Bot,
    chat_id: ChatId,
    msg_id: MessageId,
    user_id: UserId,
    user: User,
    edit: bool,
) -> super::types::HandlerResult {
    let data: UserData = try_or_reply!(user.request(), bot, chat_id);
    let attedance: AttedanceList = try_or_reply!(user.request(), bot, chat_id);
    let average_progress: AverageProgressList = try_or_reply!(user.request(), bot, chat_id);
    let from = chrono::Local::now().date_naive();

    let coins = data.gaming_points[0].points;
    let gems = data.gaming_points[1].points;
    let reg_date = NaiveDateTime::from_rus(&data.registration_date);

    let att = attedance
        .0
        .iter()
        .rfind(|c| NaiveDate::parse_from_str(&c.date, "%Y-%m-%d").unwrap() <= from);
    let attedance_points = att.and_then(|c| c.points).unwrap_or(0);
    let attedance_prev_points = att.and_then(|c| c.previous_points).unwrap_or(0);

    let attedance_sign: Option<&str> = if attedance_points > attedance_prev_points {
        Some("+")
    } else if attedance_points < attedance_prev_points {
        Some("-")
    } else {
        None
    };

    let avg = average_progress
        .0
        .iter()
        .rfind(|c| NaiveDate::parse_from_str(&c.date, "%Y-%m-%d").unwrap() <= from);
    let avg_points = avg.and_then(|c| c.points).unwrap_or(0);
    let avg_prev_points = avg.and_then(|c| c.previous_points).unwrap_or(0);

    let avg_sign: Option<&str> = if avg_points > avg_prev_points {
        Some("+")
    } else if avg_points < avg_prev_points {
        Some("-")
    } else {
        None
    };

    let text = format!(
        "Привет, {}!\n\n\
        \
        Твой айди: {}\n\
        Дата регистрации: {}\n\
        Группа: {}\n\n\
        \
        Количество топкоинов: {coins}\n\
        Количество топгемов: {gems}\n\
        Количество топмани: {}\n\n\
        \
        Твой средний балл за этот месяц: {avg_points} {}\n\
        Посещаемость в этом месяце: {attedance_points}% {}\n\n\
        \
        Это неофициальный бот. Он не связан с Компьютерной академией ТОР и разработан сторонними авторами.",
        data.full_name,
        data.student_id,
        reg_date.to_rus(),
        data.group_name,
        coins + gems,
        if let Some(sign) = avg_sign {
            format!("({}{})", sign, (avg_points - avg_prev_points).abs())
        } else {
            String::new()
        },
        if let Some(sign) = attedance_sign {
            format!(
                "({}{}%)",
                sign,
                (attedance_points - attedance_prev_points).abs()
            )
        } else {
            String::new()
        }
    );

    if edit {
        bot.edit_message_text(chat_id, msg_id, text)
            .reply_markup(super::keyboards::main_menu_kb(data.current_group_id))
            .await?;
    } else {
        bot.send_message(chat_id, text)
            .reply_markup(super::keyboards::main_menu_kb(data.current_group_id))
            .await?;
    }

    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        if let Ok(evaluates) = user.get_evaluates(user_id).await {
            if let Some(first) = evaluates.first() {
                let _ = send_evaluate(bot, chat_id, first).await;
            }
        }
    });

    Ok(())
}

pub async fn relogin_command(
    bot: Bot,
    msg: Message,
    dialogue: super::types::MyDialogue,
) -> super::types::HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Введите ваш логин и пароль от Top Academy в формате: <code>логин пароль</code>.",
    )
    .parse_mode(teloxide::types::ParseMode::Html)
    .reply_markup(super::keyboards::cancel_kb())
    .await?;

    dialogue
        .update(super::types::State::ReceiveCredentials)
        .await?;

    Ok(())
}
