use teloxide::{
    dispatching::{UpdateHandler, dialogue::InMemStorage},
    prelude::*,
};

mod auth;
pub mod callbacks;
mod commands;
pub mod keyboards;
pub mod types;

#[rustfmt::skip]
pub fn handler() -> UpdateHandler<types::HandlerError> {
    dptree::entry()
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<types::State>, types::State>()
                .branch(
                    dptree::case![types::State::ReceiveCredentials]
                        .endpoint(auth::auth)
                )
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, InMemStorage<types::State>, types::State>()
                .endpoint(callbacks::callback_handler)
        )
        .branch(
            dptree::entry()
                .filter_map_async(auth::check_user)
                .branch(
                    Update::filter_message().branch(
                        dptree::entry()
                            .filter_command::<commands::Command>()
                            .endpoint(commands::on_command),
                    ),
                )
        )
}
