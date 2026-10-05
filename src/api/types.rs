pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
}
