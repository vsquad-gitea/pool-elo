// (Server only) Routes

use crate::{
    data::{
        pool_match::{PoolMatch, PoolMatchList, UserList},
        store::DATA,
    },
    endpoints::MATCH,
    endpoints::USER,
};
use axum::{
    extract::Json,
    routing::{post, Router},
};
use std::thread;

pub fn register_routes(app: Router) -> Router {
    let app = app.route(MATCH, post(post_match));
    let app = app.route(USER, post(post_user));
    app
}

async fn post_match(Json(pool_match): Json<PoolMatch>) -> Json<PoolMatchList> {
    // Update the store with the new match
    let matches = thread::spawn(move || {
        // Get the store
        let mut data = DATA.lock().unwrap();
        (*data).matches.add_pool_match(pool_match);
        println!("{:?}", (*data).matches.pool_matches);
        (*data).matches.clone()
    })
    .join()
    .unwrap();

    Json(matches)
}

async fn post_user(user: String) -> Json<UserList> {
    // Update the store with the new match
    let users = thread::spawn(move || {
        // Get the store
        let mut data = DATA.lock().unwrap();
        let user_id = (*data).users.add_user(user);
        println!("Added new user id: {}\nAll users: {:?}", user_id, (*data).users);
        (*data).users.clone()
    })
    .join()
    .unwrap();

    Json(users)
}

