use teloxide::prelude::*;

pub mod api;
pub mod bot;
pub mod error;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    log::info!("Starting bot...");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .min_connections(2)
        .connect(
            std::env::var("DATABASE_URL")
                .expect("env DATABASE_URL not set")
                .as_str(),
        )
        .await
        .expect("Cannot connect to db");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, bot::handler())
        .dependencies(dptree::deps![pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
