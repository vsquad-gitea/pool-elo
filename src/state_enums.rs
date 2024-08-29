use std::fmt::Display;

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

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameState::None => "",
                GameState::Pool => "Pool",
                GameState::Pickleball => "Pickle Ball",
                GameState::TableTennis => "Table Tennis",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum OpenState {
    Open,
    Closed,
}
