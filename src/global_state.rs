// Not a page, global state that is shared between all pages

use perseus::{prelude::*, state::GlobalStateCreator};
use serde::{Deserialize, Serialize};

use crate::{
    models::auth::WebAuthInfo,
    state_enums::{LoginState, OpenState},
};

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "AppStateRx")]
pub struct AppState {
    #[rx(nested)]
    pub auth: AuthData,
    #[rx(nested)]
    pub modals_open: ModalOpenData,
    #[rx(nested)]
    pub style: StyleData,
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "AuthDataRx")]
pub struct AuthData {
    pub state: LoginState,
    pub pending_username: String,
    pub username: Option<String>,
    pub remember_me: Option<bool>,
    pub auth_info: Option<WebAuthInfo>,
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "ModalOpenDataRx")]
pub struct ModalOpenData {
    pub login: OpenState,
    pub register: OpenState,
    pub forgot_password: OpenState,
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "StyleDataRx")]
pub struct StyleData {
    #[rx(nested)]
    pub theme: ThemeData,
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "ThemeDataRx")]
pub struct ThemeData {
    pub current: String,
    pub pool: String,
    pub pickleball: String,
    pub table_tennis: String,
    pub default: String,
}

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}

#[engine_only_fn]
pub async fn get_build_state() -> AppState {
    AppState {
        auth: AuthData {
            state: LoginState::Unknown,
            pending_username: String::new(),
            username: None,
            remember_me: None,
            auth_info: None,
        },
        modals_open: ModalOpenData {
            login: OpenState::Closed,
            register: OpenState::Closed,
            forgot_password: OpenState::Closed,
        },
        style: StyleData {
            theme: ThemeData {
                current: "light".to_owned(),
                pool: "autumn".to_owned(),
                pickleball: "lemonade".to_owned(),
                table_tennis: "nord".to_owned(),
                default: "light".to_owned(),
            },
        },
    }
}

impl AuthDataRx {
    #[cfg(client)]
    pub fn handle_log_in(&self, auth_info: WebAuthInfo) {
        // Save new token to persistent storage
        if auth_info.remember_me {
            let storage: web_sys::Storage =
                web_sys::window().unwrap().local_storage().unwrap().unwrap();
            let value = serde_json::to_string(&auth_info).unwrap();
            storage.set_item("auth", &value).unwrap();
        }
        // Save into session storage always
        let storage: web_sys::Storage = web_sys::window()
            .unwrap()
            .session_storage()
            .unwrap()
            .unwrap();
        let value = serde_json::to_string(&auth_info).unwrap();
        storage.set_item("auth", &value).unwrap();

        // Save token to session storage
        self.username.set(Some(auth_info.username.clone()));
        self.remember_me.set(Some(auth_info.remember_me));
        self.auth_info.set(Some(auth_info));
        self.state.set(LoginState::Authenticated);
    }
    #[cfg(client)]
    pub fn handle_log_out(&self) {
        // Delete persistent storage
        // TODO -> handle error if local storage is not readable in browser
        let storage: web_sys::Storage =
            web_sys::window().unwrap().local_storage().unwrap().unwrap();
        storage.remove_item("auth").unwrap();
        let storage: web_sys::Storage = web_sys::window()
            .unwrap()
            .session_storage()
            .unwrap()
            .unwrap();
        storage.remove_item("auth").unwrap();
        // Update state
        self.auth_info.set(None);
        self.username.set(None);
        self.remember_me.set(None);
        self.state.set(LoginState::NotAuthenticated);
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
        // TODO handle error case better
        // Save new token to persistent storage
        let storage: web_sys::Storage =
            web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let saved_auth = storage.get("auth").unwrap();
        match saved_auth {
            Some(auth_info) => {
                // TODO check if session is expiring
                let auth_info = serde_json::from_str(&auth_info).unwrap();
                self.handle_log_in(auth_info);
            }
            None => {
                // Try session storage
                let storage: web_sys::Storage = web_sys::window()
                    .unwrap()
                    .session_storage()
                    .unwrap()
                    .unwrap();
                let saved_auth = storage.get("auth").unwrap();
                match saved_auth {
                    Some(auth_info) => {
                        let auth_info = serde_json::from_str(&auth_info).unwrap();
                        self.handle_log_in(auth_info);
                    }
                    None => {
                        self.state.set(LoginState::NotAuthenticated);
                    }
                }
            }
        }
    }
}
