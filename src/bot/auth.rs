use crate::types::*;
use anyhow::Context;
use std::sync::Arc;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::MessageId};

pub async fn check_user(
    bot: Bot,
    update: Update,
    pool: sqlx::PgPool,
    storage: Arc<InMemStorage<super::types::State>>,
) -> Option<User> {
    match User::get(&pool, update.from()?.id.0 as i64).await {
        Some(u) => Some(u),
        None => {
            let chat_id = update.chat()?.id;

            bot.send_message(chat_id,
                "Введите ваш логин и пароль от Top Academy в формате: <code>логин пароль</code>.\n\
                Это неофициальный бот. Он не связан с Компьютерной академией ТОР и разработан сторонними авторами." 
            ).parse_mode(teloxide::types::ParseMode::Html).reply_markup(super::keyboards::cancel_kb()).await.ok()?;

            let dialogue: super::types::MyDialogue = Dialogue::new(storage, chat_id);
            dialogue
                .update(super::types::State::ReceiveCredentials)
                .await
                .ok()?;

            return None;
        }
    }
}

pub async fn auth(
    bot: Bot,
    msg: Message,
    pool: sqlx::PgPool,
    dialogue: super::types::MyDialogue,
) -> super::types::HandlerResult {
    let user_id = msg.from.as_ref().context("Не юзер")?.id.0 as i64;
    let parts: Vec<&str> = msg.text().unwrap_or("").split_whitespace().collect();

    if let [login, password] = parts[..] {
        let user =
            match crate::types::User::new(&pool, user_id, login.to_string(), password.to_string())
                .await
            {
                Ok(u) => u,
                Err(crate::error::ApiError::FailedLogin) => {
                    bot.send_message(
                        msg.chat.id,
                        "Не удалось войти. Проверьте данные и попробуйте снова:",
                    )
                    .reply_markup(super::keyboards::cancel_kb())
                    .await?;
                    return Ok(());
                }
                Err(crate::error::ApiError::Network(err)) => {
                    log::error!("{:?}", err);
                    bot.send_message(
                        msg.chat.id,
                        "Ошибка на стороне серверов Академии Топ! Попробуйте позже.",
                    )
                    .reply_markup(super::keyboards::cancel_kb())
                    .await?;
                    return Ok(());
                }
                Err(crate::error::ApiError::Database(err)) => {
                    log::error!("{:?}", err);
                    bot.send_message(
                        msg.chat.id,
                        "Произошла ошибка на нашей стороне. Попробуйте позже.",
                    )
                    .reply_markup(super::keyboards::cancel_kb())
                    .await?;
                    return Ok(());
                }
            };

        bot.delete_messages(msg.chat.id, [msg.id, MessageId { 0: msg.id.0 - 1 }])
            .await?;
        dialogue.exit().await?;

        bot.send_message(
            msg.chat.id,
            "Вы успешно авторизованы! Перехожу в главное меню...",
        )
        .await?;
    } else {
        bot.send_message(
            msg.chat.id,
            "Формат: <code>логин пароль</code>. Попробуйте еще раз:",
        )
        .parse_mode(teloxide::types::ParseMode::Html)
        .reply_markup(super::keyboards::cancel_kb())
        .await?;
    }

    Ok(())
}
