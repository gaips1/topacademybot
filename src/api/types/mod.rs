use serde::de::DeserializeOwned;

pub struct User {
    pub username: String,
    pub password: String,
}

pub trait Endpoint: DeserializeOwned {
    const PATH: &'static str;
}

#[derive(serde::Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
}

// --------------------------------------------

mod userdata;
mod rewards;
mod leaderboard;
mod activity;
mod homework;

pub use userdata::*;
pub use rewards::*;
pub use leaderboard::*;
pub use activity::*;
pub use homework::*;