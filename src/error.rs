use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Не удалось отправить запрос: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Не удалось отправить запрос к базе данных: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Не удалось авторизовать пользователя")]
    FailedLogin,

    #[error("Ошибка шифрования: {0}")]
    Crypto(String),
}

impl ApiError {
    pub fn user_message(&self) -> &'static str {
        match self {
            ApiError::Network(err) => {
                log::error!("{:#?}", err);
                "Сервис сейчас недоступен. Попробуйте чуть позже."
            }
            ApiError::Database(err) => {
                log::error!("{:#?}", err);
                "Внутренняя ошибка. Мы уже разбираемся."
            }
            ApiError::FailedLogin => {
                "Не получилось войти в Ваш аккаунт. Введите новые данные используя команду /relogin."
            }
            ApiError::Crypto(err) => {
                log::error!("{:#?}", err);
                "Внутренняя ошибка безопасности. Попробуйте позже."
            }
        }
    }
}
