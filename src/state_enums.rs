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

impl ToString for GameState {
    fn to_string(&self) -> String {
        match self {
            GameState::None => String::new(),
            GameState::Pool => "Pool".to_owned(),
            GameState::Pickleball => "Pool".to_owned(),
            GameState::TableTennis => "Pool".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum OpenState {
    Open,
    Closed,
}
