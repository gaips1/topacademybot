use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};
use std::{num::NonZeroU32, sync::LazyLock, time::Duration};
use crate::types::*;

static LIMITER: LazyLock<DefaultKeyedRateLimiter<u64>> =
    LazyLock::new(|| RateLimiter::keyed(Quota::per_second(NonZeroU32::new(3).unwrap())));

static WARN_LIMITER: LazyLock<DefaultKeyedRateLimiter<u64>> = LazyLock::new(|| {
    RateLimiter::keyed(Quota::with_period(Duration::from_secs(5)).unwrap())
});

pub async fn check_rate_limit(bot: Bot, update: Update) -> bool {
    let (Some(from), Some(chat)) = (update.from(), update.chat()) else {
        return false;
    };

    if LIMITER.check_key(&from.id.0).is_ok() {
        return true;
    }

    if WARN_LIMITER.check_key(&from.id.0).is_ok() {
        let _ = bot
            .send_message(chat.id, "Вы слишком часто отправляете запросы. Попробуйте позже.")
            .await;
    }
    false
}

pub fn run_clean_limiters_task() {
    tokio::spawn(async {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            LIMITER.retain_recent();
            LIMITER.shrink_to_fit();
            WARN_LIMITER.retain_recent();
            WARN_LIMITER.shrink_to_fit();
        }
    });
}