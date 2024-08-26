// Not a page, global state that is shared between all pages

use perseus::{prelude::*, state::GlobalStateCreator};
use serde::{Deserialize, Serialize};

use crate::{
    models::auth::Claims,
    state_enums::{LoginState, OpenState},
};

cfg_if::cfg_if! {
    if #[cfg(engine)] {

    }
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "AppStateRx")]
pub struct AppState {
    #[rx(nested)]
    pub auth: AuthData,
    #[rx(nested)]
    pub modals_open: ModalOpenData,
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "AuthDataRx")]
pub struct AuthData {
    pub state: LoginState,
    pub username: Option<String>,
    pub claims: Claims,
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "ModalOpenDataRx")]
pub struct ModalOpenData {
    pub login: OpenState,
}

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}

#[engine_only_fn]
pub async fn get_build_state() -> AppState {
    AppState {
        auth: AuthData {
            state: LoginState::Unknown,
            username: None,
            claims: Claims {
                sub: "".to_owned(),
                exp: 0,
            },
        },
        modals_open: ModalOpenData {
            login: OpenState::Closed,
        },
    }
}

// Client only code to check if they're authenticated
#[cfg(client)]
impl AuthDataRx {
    pub fn detect_state(&self) {
        // If the user is in a known state, return
        if let LoginState::Authenticated | LoginState::NotAuthenticated = *self.state.get() {
            return;
        }
        // TODO -> Get state from storage
        self.state.set(LoginState::NotAuthenticated);
    }
}
