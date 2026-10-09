use crate::api::types::User;
use teloxide::{dispatching::UpdateHandler, macros::BotCommands, prelude::*};

mod commands;
pub mod types;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Start,
    Relogin,
}

async fn load_user(update: Update, pool: sqlx::PgPool) -> Option<User> {
    User::get(&pool, update.from()?.id.0 as i64).await
}

#[rustfmt::skip]
pub fn handler() -> UpdateHandler<types::HandlerError> {
    dptree::entry()
        // .branch(
        // dptree::entry()
        //     .branch(
        //         Update::filter_message().branch(
        //             dptree::entry()
        //                 .filter_command::<Command>()
        //                 .endpoint(commands::on_command),
        //         ),
        //     )
        // )
        .branch(
            dptree::entry().filter_map_async(load_user).branch(
                Update::filter_message().branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(commands::on_command),
                ),
            ),
        )
}
