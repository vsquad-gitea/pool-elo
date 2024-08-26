use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    pub token: String,
    #[serde(with = "ts_seconds")]
    pub expires: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
