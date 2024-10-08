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

// For client local storage and session storage
#[derive(Serialize, Deserialize, Clone)]
pub struct WebAuthInfo {
    pub token: String,
    #[serde(with = "ts_seconds")]
    pub expires: DateTime<Utc>,
    pub username: String,
    pub remember_me: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub nickname: String,
    pub registration_code: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ForgotPasswordRequest {
    pub username: String,
    pub contact_info: String,
}
