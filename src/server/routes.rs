// (Server only) Routes
use crate::{endpoints::USER, entity::user};
use axum::{
    extract::Json,
    routing::{post, Router},
};

pub fn register_routes(app: Router) -> Router {
    let app = app.route(USER, post(post_user));
    app
}

async fn post_user(user: String) -> Json<user::Model> {
    // Update the store with the new match
    todo!()
}
