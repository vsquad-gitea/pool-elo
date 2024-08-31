use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum LoginState {
    Authenticated,
    NotAuthenticated,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ContentState {
    None,
    Pool,
    Pickleball,
    TableTennis,
}

impl Display for ContentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ContentState::None => "",
                ContentState::Pool => "Pool",
                ContentState::Pickleball => "Pickle Ball",
                ContentState::TableTennis => "Table Tennis",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum OpenState {
    Open,
    Closed,
}
