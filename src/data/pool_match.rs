use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PoolMatch {
    pub players: Vec<String>,
    pub winner: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct PoolMatchList {
    pub pool_matches: Vec<PoolMatch>,
}
