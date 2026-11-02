use super::{types::*, *};
use dashmap::DashMap;
use serde::de::IgnoredAny;
use std::sync::LazyLock;
use tokio::time::Instant;

static EVALUATE_CACHE: LazyLock<DashMap<UserId, (Instant, EvaluateList)>> =
    LazyLock::new(DashMap::new);

const COOLDOWN: Duration = Duration::from_secs(900);

impl User {
    pub async fn get_evaluates(&self, user_id: UserId) -> Result<EvaluateList, ApiError> {
        if EVALUATE_CACHE.len() > 10_000 {
            EVALUATE_CACHE.retain(|_, (t, list)| !list.is_empty() || t.elapsed() < COOLDOWN);
        }

        let mut entry = EVALUATE_CACHE.entry(user_id).or_insert_with(|| {
            (
                Instant::now()
                    .checked_sub(COOLDOWN)
                    .unwrap_or_else(Instant::now),
                Vec::new(),
            )
        });

        if !entry.1.is_empty() {
            return Ok(entry.1.clone());
        }

        if entry.0.elapsed() < COOLDOWN {
            return Ok(Vec::new());
        }

        entry.0 = Instant::now();
        drop(entry);

        let list: EvaluateList = self.request().await?;

        if let Some(mut entry) = EVALUATE_CACHE.get_mut(&user_id) {
            entry.1 = list.clone();
        }

        Ok(list)
    }

    pub async fn evaluate_lesson(
        &self,
        user_id: UserId,
        key: &str,
        mark_lesson: u8,
        mark_teach: u8,
    ) -> Result<EvaluateList, ApiError> {
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

        self.request_with_options::<IgnoredAny>(request_builder)
            .await?;

        if let Some(mut entry) = EVALUATE_CACHE.get_mut(&user_id) {
            entry.1.retain(|e| e.key != key);
            return Ok(entry.1.clone());
        }

        Ok(Vec::new())
    }
}
