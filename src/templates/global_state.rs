// Not a page, global state that is shared between all pages

use perseus::{prelude::*, state::GlobalStateCreator};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(engine)] {
        use std::thread;
        use std::ops::Deref;
    }
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "AppStateRx")]
pub struct AppState {}

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new()
        .build_state_fn(get_build_state)
        .request_state_fn(get_request_state)
}

#[engine_only_fn]
fn get_state() -> AppState {
    AppState {}
}

#[engine_only_fn]
pub async fn get_build_state() -> AppState {
    get_state()
}

#[engine_only_fn]
pub async fn get_request_state(_req: Request) -> AppState {
    get_state()
}
