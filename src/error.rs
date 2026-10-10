use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Не удалось отправить запрос: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Не удалось отправить запрос к базе данных: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Не удалось авторизовать пользователя")]
    FailedLogin,
}
