use crate::{
    entity::{prelude::*, user},
    models::{auth::RegisterRequest, generic::GenericResponse},
    server::server_state::ServerState,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher,
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

pub async fn post_register_user(
    State(state): State<ServerState>,
    Json(register_info): Json<RegisterRequest>,
) -> (StatusCode, Json<GenericResponse>) {
    // TODO -> update to use env, maybe prevent brute force too
    if register_info.registration_code != "ferris" {
        return (
            StatusCode::UNAUTHORIZED,
            Json(GenericResponse::err("Incorrect registration code")),
        );
    }

    // See if username already exists
    let username = register_info.username;
    let existing_user: Option<user::Model> = User::find()
        .filter(user::Column::Username.eq(username.clone()))
        .one(&state.db_conn)
        .await
        .unwrap();
    if existing_user.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(GenericResponse::err("Username already exists")),
        );
    }

    // Generate password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(register_info.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let phc_string = PasswordHash::new(&password_hash).unwrap().to_string();

    // If the username doen't exist, create the user
    let new_user = user::ActiveModel {
        username: Set(username),
        password_hash_and_salt: Set(phc_string),
        nickname: Set({
            if register_info.nickname == "" {
                None
            } else {
                Some(register_info.nickname)
            }
        }),
        creation_time: Set(Utc::now().naive_utc()),
        last_active_time: Set(Utc::now().naive_utc()),
        is_admin: Set(false),
        email: Set({
            if register_info.email == "" {
                None
            } else {
                Some(register_info.email)
            }
        }),
        avatar: Set(None),
        forgot_password_request: Set(None),
        ..Default::default()
    };
    let db_resp = user::Entity::insert(new_user).exec(&state.db_conn).await;
    match db_resp {
        Ok(_) => {}
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(GenericResponse::err(err.to_string().as_str())),
            );
        }
    };

    return (StatusCode::OK, Json(GenericResponse::ok()));
}
