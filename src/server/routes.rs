// (Server only) Routes

use crate::{
    data::{
        pool_match::{PoolMatch, PoolMatchList},
        store::DATA,
    },
    endpoints::MATCH,
};
use axum::{
    extract::Json,
    routing::{post, Router},
};
use std::thread;

pub fn register_routes(app: Router) -> Router {
    let app = app.route(MATCH, post(post_match));
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
