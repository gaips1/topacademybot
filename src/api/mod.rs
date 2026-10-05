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

mod types;

const BASE_URL: &str = "https://msapi.top-academy.ru/api/v2";

pub static REQWEST_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("не удалось создать HTTP-клиент")
});

static TOKENS: LazyLock<DashMap<String, Arc<Mutex<Option<String>>>>> = LazyLock::new(DashMap::new);

impl types::User {
    async fn cached_token(&self) -> Option<String> {
        let slot = TOKENS.get(&self.username).map(|s| s.clone())?;
        let guard = slot.lock().await;
        guard.clone()
    }

    async fn login(&self) -> Result<String, crate::error::ApiError> {
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
            reqwest::StatusCode::UNAUTHORIZED => {
                return Err(crate::error::ApiError::FailedLogin);
            }
            _ => {
                resp.error_for_status_ref()?;
            }
        }

        let data: types::AuthResponse = resp.json().await?;

        Ok(data.access_token)
    }

    async fn refresh_token(&self, stale: Option<&str>) -> Result<String, ApiError> {
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
    ) -> Result<T, crate::error::ApiError> {
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
                    return Err(crate::error::ApiError::FailedLogin);
                }
            }
        }

        Ok(resp.error_for_status()?.json::<T>().await?)
    }

    pub async fn request<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, crate::error::ApiError> {
        todo!() // get запрос по эндпоинту
    }
}
