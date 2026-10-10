use serde::de::DeserializeOwned;

#[derive(Clone)]
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

mod activity;
mod homework;
mod leaderboard;
mod rewards;
mod userdata;

pub use activity::*;
pub use homework::*;
pub use leaderboard::*;
pub use rewards::*;
pub use userdata::*;
