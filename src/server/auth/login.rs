use crate::{
    entity::{
        prelude::*,
        user::{self},
    },
    models::{
        auth::{Claims, LoginInfo, LoginResponse},
        generic::GenericResponse,
    },
    server::server_state::ServerState,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub async fn credentials_are_correct(
    username: &str,
    password: &str,
    state: &ServerState,
) -> Result<(), String> {
    // Get user
    let existing_user: Option<user::Model> = User::find()
        .filter(user::Column::Username.eq(username))
        .one(&state.db_conn)
        .await
        .unwrap();
    let hash_to_check: String = match existing_user {
        Some(user) => user.password_hash_and_salt,
        None => {
            // @todo make dummy password hash
            return Err("Username doesn't exist".to_owned());
        }
    };

    match Argon2::default().verify_password(
        password.as_bytes(),
        &PasswordHash::new(hash_to_check.as_str()).unwrap(),
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid credentials".to_owned()),
    }
}

pub async fn post_login_user(
    State(state): State<ServerState>,
    Json(login_info): Json<LoginInfo>,
) -> (
    StatusCode,
    Result<Json<LoginResponse>, Json<GenericResponse>>,
) {
    let user_authenticated =
        credentials_are_correct(&login_info.username, &login_info.password, &state);

    match user_authenticated.await {
        Err(why) => (
            StatusCode::UNAUTHORIZED,
            Err(Json(GenericResponse::err(why.as_str()))),
        ),
        Ok(_) => {
            let expires = match login_info.remember_me {
                true => chrono::Utc::now() + chrono::Duration::days(365),
                false => chrono::Utc::now() + chrono::Duration::days(1),
            };

            let claims = Claims {
                sub: login_info.username.clone(),
                exp: expires.timestamp() as usize,
            };
            // @todo change secret
            let token = match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("secret".as_ref()),
            ) {
                Ok(token) => token,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Err(Json(GenericResponse::err("Failed to get token"))),
                    )
                }
            };

            (StatusCode::OK, Ok(Json(LoginResponse { token, expires })))
        }
    }
}

pub async fn post_test_login(
    State(_): State<ServerState>,
    header_map: HeaderMap,
) -> Result<Json<String>, StatusCode> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
                let token = auth_header_str.trim_start_matches("Bearer ").to_string();
                // @todo change secret
                if let Ok(_) = decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret("secret".as_ref()),
                    &Validation::default(),
                ) {
                    return Ok(Json("Logged in".to_owned()));
                }
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
