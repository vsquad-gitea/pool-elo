use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum LoginState {
    Authenticated,
    NotAuthenticated,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GameState {
    None,
    Pool,
    Pickleball,
    TableTennis,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum OpenState {
    Open,
    Closed,
}
