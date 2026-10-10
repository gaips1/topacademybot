use teloxide::{macros::BotCommands, prelude::*};
use crate::types::*;

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
    user: User
) -> super::types::HandlerResult {
    match cmd {
        Command::Start => start_command(bot, msg, user).await?,
        Command::Relogin => {}
    }
    Ok(())
}

pub async fn start_command(
    bot: Bot,
    msg: Message,
    user: User
) -> super::types::HandlerResult {
    let data: UserData = user.request().await?;

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
        data.full_name, data.student_id,
        data.registration_date,
        data.group_name, coins + gems
    );

    bot.send_message(msg.chat.id, text).reply_markup(super::keyboards::main_menu_kb()).await?;
    Ok(())
}
