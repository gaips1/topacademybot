use crate::error::ApiError;
use dashmap::DashMap;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::json;
use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};
use tokio::sync::Mutex;

pub mod types;

pub const BASE_URL: &str = "https://msapi.top-academy.ru/api/v2";

pub static REQWEST_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("не удалось создать HTTP-клиент")
});

static TOKENS: LazyLock<DashMap<String, Arc<Mutex<Option<String>>>>> = LazyLock::new(DashMap::new);

impl types::User {
    pub async fn cached_token(&self) -> Option<String> {
        let slot = TOKENS.get(&self.username).map(|s| s.clone())?;
        let guard = slot.lock().await;
        guard.clone()
    }

    pub async fn login(&self) -> Result<String, ApiError> {
        let resp = REQWEST_CLIENT
            .post(format!("{}/auth/login", BASE_URL))
            .header("Content-Type", "application/json")
            .header("Referer", "https://journal.top-academy.ru/")
            .json(&json!({
                "application_key": "6a56a5df2667e65aab73ce76d1dd737f7d1faef9c52e8b8c55ac75f565d8e8a6",
                "id_city": null,
                "username": self.username,
                "password": self.password
            }))
            .send()
            .await?;

        match resp.status() {
            reqwest::StatusCode::UNAUTHORIZED | StatusCode::UNPROCESSABLE_ENTITY => {
                return Err(crate::error::ApiError::FailedLogin);
            }
            _ => {
                resp.error_for_status_ref()?;
            }
        }

        let data: types::AuthResponse = resp.json().await?;

        Ok(data.access_token)
    }

    pub async fn refresh_token(&self, stale: Option<&str>) -> Result<String, ApiError> {
        let slot = TOKENS.entry(self.username.clone()).or_default().clone();
        let mut guard = slot.lock().await;

        if let Some(current) = guard.as_deref() {
            if Some(current) != stale {
                return Ok(current.to_owned());
            }
        }

        let token = self.login().await?;
        *guard = Some(token.clone());
        Ok(token)
    }

    pub async fn request_with_options<T: DeserializeOwned>(
        &self,
        request_builder: reqwest::RequestBuilder,
    ) -> Result<T, ApiError> {
        let token = match self.cached_token().await {
            Some(t) => t,
            None => self.refresh_token(None).await?,
        };

        let retry_builder = request_builder.try_clone();
        let mut resp = request_builder.bearer_auth(&token).send().await?;

        if resp.status() == StatusCode::UNAUTHORIZED {
            if let Some(retry) = retry_builder {
                let new_token = self.refresh_token(Some(&token)).await?;
                resp = retry.bearer_auth(&new_token).send().await?;

                if resp.status() == StatusCode::UNAUTHORIZED {
                    return Err(ApiError::FailedLogin);
                }
            }
        }

        if resp.status().is_server_error() {
            let err = resp.error_for_status_ref().unwrap_err();
            let body = resp.text().await.unwrap_or_default();
            log::error!("{} {}: {}", err.url().map(|u| u.as_str()).unwrap_or(""), err.status().map(|s| s.as_u16()).unwrap_or(0), body);
            return Err(err.into());
        }

        Ok(resp.error_for_status()?.json::<T>().await?)
    }

    pub async fn request_path<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let request_builder = REQWEST_CLIENT
            .get(format!("{BASE_URL}{}", path))
            .header("Accept", "application/json, text/plain, */*")
            .header("Referer", "https://journal.top-academy.ru/")
            .header("Content-Type", "application/json");

        self.request_with_options::<T>(request_builder).await
    }

    pub async fn request<T: types::Endpoint>(&self) -> Result<T, ApiError> {
        let request_builder = REQWEST_CLIENT
            .get(format!("{BASE_URL}{}", T::PATH))
            .header("Accept", "application/json, text/plain, */*")
            .header("Referer", "https://journal.top-academy.ru/")
            .header("Content-Type", "application/json");

        self.request_with_options::<T>(request_builder).await
    }

    pub async fn get(pool: &sqlx::PgPool, id: i64) -> Option<Self> {
        sqlx::query_as!(
            Self,
            r#"SELECT username, password FROM users WHERE id = $1;"#,
            id
        )
        .fetch_one(pool)
        .await
        .ok()
    }

    pub async fn new(
        pool: &sqlx::PgPool,
        user_id: i64,
        username: String,
        password: String,
    ) -> Result<Self, ApiError> {
        let user = Self { username, password };

        match user.cached_token().await {
            Some(t) => t,
            None => user.refresh_token(None).await?,
        };

        sqlx::query!(
            "INSERT INTO users (id, username, password) \
            VALUES ($1, $2, $3) \
            ON CONFLICT (id) DO UPDATE \
            SET username = EXCLUDED.username, \
                password = EXCLUDED.password;",
            user_id,
            &user.username,
            &user.password
        )
        .execute(pool)
        .await?;

        Ok(user)
    }

    pub async fn create_homework(
        &self,
        homework_id: u64,
        file_data: Option<Vec<u8>>,
        filename: Option<String>,
        spent_hour: u8,
        spent_min: u8,
        mark: u8,
        answer_text: Option<String>,
    ) -> Result<(), ApiError> {
        let mut form_data = reqwest::multipart::Form::new()
            .text("id", homework_id.to_string());

        if let (Some(data), Some(name)) = (file_data, filename) {
            let mime = mime_guess::from_path(&name).first_or_octet_stream();
            form_data = form_data.part(
                "file",
                reqwest::multipart::Part::bytes(data)
                    .file_name(name)
                    .mime_str(mime.as_ref())?,
            );
        }

        form_data = form_data
            .text("answerText", answer_text.clone().unwrap_or_default())
            .text("spentTimeHour", format!("{:02}", spent_hour))
            .text("spentTimeMin", format!("{:02}", spent_min));

        let request_builder = REQWEST_CLIENT
            .post(format!("{BASE_URL}/homework/operations/create"))
            .header("Accept", "application/json, text/plain, */*")
            .header("Referer", "https://journal.top-academy.ru/")
            .multipart(form_data);

        self.request_with_options::<super::types::HomeworkStud>(request_builder)
            .await?;

        let form_data = reqwest::multipart::Form::new().text(
            "EvaluationHomeworkForm",
            format!(
                "{{\
                    \"id\": null, \"idDomZad\": {homework_id},\
                    \"mark\": {mark}, \"comment\": \"{}\"\
                }}",
                answer_text.unwrap_or_default()
            ),
        );

        let request_builder = REQWEST_CLIENT
            .post(format!("{BASE_URL}/homework/evaluation/operations/save"))
            .header("Accept", "application/json, text/plain, */*")
            .header("Referer", "https://journal.top-academy.ru/")
            .multipart(form_data);

        self.request_with_options::<super::types::SendedHomework>(request_builder)
            .await?;

        Ok(())
    }
}
