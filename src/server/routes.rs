// (Server only) Routes
use crate::{
    endpoints::{MATCH, USER},
    entity::{game, user},
};
use axum::{
    extract::Json,
    routing::{post, Router},
};

pub fn register_routes(app: Router) -> Router {
    let app = app.route(USER, post(post_user));
    app.route(MATCH, post(post_match))
}

async fn post_user(_user: String) -> Json<user::Model> {
    // Update the store with the new match
    todo!()
}

async fn post_match(_user: String) -> Json<game::Model> {
    // Update the store with the new match
    todo!()
}
