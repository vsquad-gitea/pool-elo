// (Server only) Routes
use crate::endpoints::{ADD_MATCH, FORGOT_PASSWORD, LOGIN, LOGIN_TEST, REGISTER};
use axum::routing::{post, Router};

use super::{
    auth::{
        forgot_password::post_forgot_password,
        login::{post_login_user, post_test_login},
        register::post_register_user,
    },
    pool::add_match::post_game_result,
    server_state::ServerState,
};

pub fn get_api_router(state: ServerState) -> Router {
    Router::new()
        .route(REGISTER, post(post_register_user))
        .route(LOGIN, post(post_login_user))
        .route(ADD_MATCH, post(post_game_result))
        .route(LOGIN_TEST, post(post_test_login))
        .route(FORGOT_PASSWORD, post(post_forgot_password))
        .with_state(state)
}
