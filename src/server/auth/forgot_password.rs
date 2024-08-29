use crate::{
    entity::{prelude::*, user},
    models::{auth::ForgotPasswordRequest, generic::GenericResponse},
    server::server_state::ServerState,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};

pub async fn post_forgot_password(
    State(state): State<ServerState>,
    Json(password_request): Json<ForgotPasswordRequest>,
) -> (StatusCode, Json<GenericResponse>) {
    // Get user
    let existing_user: Option<user::Model> = User::find()
        .filter(user::Column::Username.eq(password_request.username))
        .one(&state.db_conn)
        .await
        .unwrap();
    match existing_user {
        Some(user) => {
            let mut user = user.into_active_model();
            user.forgot_password_request = Set(Some(password_request.contact_info));
            (StatusCode::OK, Json(GenericResponse::ok()))
        }
        None => (
            StatusCode::BAD_REQUEST,
            Json(GenericResponse::err("Username doesn't exist")),
        ),
    }
}
