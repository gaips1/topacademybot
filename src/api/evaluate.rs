use super::{types::*, *};
use dashmap::DashMap;
use serde::de::IgnoredAny;
use std::sync::LazyLock;
use tokio::time::Instant;

static EVALUATE_LAST_CHECK: LazyLock<DashMap<UserId, Instant>> = LazyLock::new(DashMap::new);

const COOLDOWN: Duration = Duration::from_secs(900);

impl User {
    async fn get_evaluates(&self, user_id: UserId) -> Result<EvaluateList, ApiError> {
        if EVALUATE_LAST_CHECK.len() > 10_000 {
            EVALUATE_LAST_CHECK.retain(|_, t| t.elapsed() < COOLDOWN);
        }

        let mut last = EVALUATE_LAST_CHECK.entry(user_id).or_insert_with(|| {
            Instant::now()
                .checked_sub(COOLDOWN)
                .unwrap_or_else(Instant::now)
        });

        if last.elapsed() < COOLDOWN {
            return Ok(Vec::new());
        }

        *last = Instant::now();
        drop(last);

        Ok(self.request().await?)
    }

    async fn evaluate_lesson(&self, key: String, mark_lesson: u8, mark_teach: u8) -> Result<(), ApiError> {
        let request_builder = REQWEST_CLIENT
            .post(format!("{BASE_URL}/feedback/students/evaluate-lesson"))
            .json(&json!({
                "key": key,
                "mark_lesson": mark_lesson,
                "mark_teach": mark_teach,
                "tags_lesson": [],
                "tags_teach": [],
                "comment_lesson": "",
                "comment_teach": ""
            }));

        self.request_with_options::<IgnoredAny>(request_builder).await?;

        Ok(())
    }
}
