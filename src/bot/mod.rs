use teloxide::{
    dispatching::{UpdateHandler, dialogue::InMemStorage},
    prelude::*,
};

mod auth;
pub mod callbacks;
mod commands;
pub mod helpers;
pub mod keyboards;
pub mod types;

#[rustfmt::skip]
pub fn handler() -> UpdateHandler<types::HandlerError> {
    dptree::entry()
        .enter_dialogue::<Update, InMemStorage<types::State>, types::State>()
        .branch(
            Update::filter_message()
                .branch(
                    dptree::case![types::State::ReceiveCredentials]
                        .endpoint(auth::auth)
                )
        )
        .branch(
            Update::filter_callback_query()
                .filter(|q: CallbackQuery| q.data.as_deref() == Some("cancel"))
                .endpoint(callbacks::cancel::cancel_callback_handler)
        )
        .branch(
            dptree::entry()
                .filter_map_async(auth::check_user)
                .branch(
                    Update::filter_message().branch(
                        dptree::entry()
                            .branch(
                                dptree::entry()
                                    .filter_command::<commands::Command>()
                                    .endpoint(commands::on_command)
                            )
                            .branch(
                                dptree::case![types::State::HomeworkReceiveTime { homework_id, evaluate }]
                                    .endpoint(callbacks::homeworks::upload::receive_time)
                            )
                            .branch(
                                dptree::case![types::State::HomeworkReceiveTextAnswer { homework_id, evaluate, time_hours, time_minutes }]
                                    .endpoint(callbacks::homeworks::upload::receive_text_answer)
                            )
                            .branch(
                                dptree::case![types::State::HomeworkReceiveFileAnswer { homework_id, evaluate, time_hours, time_minutes, text_answer }]
                                    .endpoint(callbacks::homeworks::upload::receive_file_answer)
                            )
                    ),
                )
                .branch(
                    Update::filter_callback_query()
                        .endpoint(callbacks::callback_handler)
                )
        )
}
