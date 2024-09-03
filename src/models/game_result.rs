use serde::{Deserialize, Serialize};

use crate::entity::sea_orm_active_enums::GameType;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameResult {
    pub winner: String,
    pub loser: String,
    pub game_type: GameType,
}
