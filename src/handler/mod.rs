use axum::{
    extract::Json,
};
use crate::data::pool_match::{PoolMatch, PoolMatchList};
use std::thread;
use crate::data::store::DATA;

pub async fn post_match(Json(pool_match): Json<PoolMatch>) -> Json<PoolMatchList> {
    // Update the store with the new match
    let matches = thread::spawn(move || {
        // Get the store
        let mut store = DATA.lock().unwrap();
        // Add the match
        (*store).matches.pool_matches.push(pool_match);
        // Return all pool matches
        (*store).matches.clone()
    }).join().unwrap();

    Json(matches)
}
