use std::time::Duration;

use crate::{bot::callbacks::evaluates::send_evaluate, types::*};
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
    let data = match user.request::<UserData>().await {
        Ok(data) => data,
        Err(err) => {
            let _ = bot.send_message(chat_id, err.user_message()).await;
            return Ok(());
        }
    };

    let coins = data.gaming_points[0].points;
    let gems = data.gaming_points[1].points;

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
        Это неофициальный бот. Он не связан с Компьютерной академией ТОР и разработан сторонними авторами.",
        data.full_name,
        data.student_id,
        data.registration_date,
        data.group_name,
        coins + gems
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

    sleep(Duration::from_secs(2)).await;
    if let Ok(evaluates) = user.get_evaluates(user_id).await {
        if let Some(first) = evaluates.first() {
            let _ = send_evaluate(bot, chat_id, first).await;
        }
    }

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
