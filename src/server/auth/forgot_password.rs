use crate::{models::auth::ForgotPasswordRequest, server::server_state::ServerState};
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
};
use sea_orm::DatabaseConnection;

pub async fn post_forgot_password(
    State(state): State<ServerState>,
    Json(password_request): Json<ForgotPasswordRequest>,
) -> StatusCode {
    StatusCode::OK
}