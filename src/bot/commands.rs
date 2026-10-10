use teloxide::{macros::BotCommands, prelude::*};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
    Relogin,
}

pub async fn on_command(
    bot: Bot,
    msg: Message,
    pool: sqlx::PgPool,
    cmd: Command,
) -> super::types::HandlerResult {
    match cmd {
        Command::Start => start_command(bot, msg, pool).await?,
        Command::Relogin => {}
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
