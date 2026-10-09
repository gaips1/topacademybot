use teloxide::prelude::*;

pub async fn on_command(
    bot: Bot,
    msg: Message,
    pool: sqlx::PgPool,
    cmd: super::Command,
) -> super::types::HandlerResult {
    match cmd {
        super::Command::Start => start_command(bot, msg, pool).await?,
        super::Command::Relogin => {}
    }
    Ok(())
}

pub async fn start_command(
    bot: Bot,
    msg: Message,
    pool: sqlx::PgPool,
) -> super::types::HandlerResult {
    bot.send_message(msg.chat.id, "Бурмалда").await?;
    Ok(())
}
