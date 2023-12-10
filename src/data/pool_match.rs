use crate::data::user::PlayerId;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type MatchId = u32;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MatchType {
    Standard8Ball,
    Standard9Ball,
    CutThroat,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MatchData {
    pub type_: MatchType,
    pub winners: Vec<PlayerId>,
    pub losers: Vec<PlayerId>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PoolMatch {
    pub id: MatchId,
    pub data: MatchData,
    #[serde(with = "ts_seconds")]
    pub time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PoolMatchList {
    pub pool_matches: Vec<PoolMatch>,
    pub max_id: MatchId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserList{
    pub users: Vec<String>,
}

impl PoolMatch {
    pub fn new(data: MatchData, time: DateTime<Utc>) -> PoolMatch {
        PoolMatch { id: 0, data, time }
    }
}

impl PoolMatchList {
    pub fn new() -> PoolMatchList {
        PoolMatchList {
            pool_matches: vec![],
            max_id: 0,
        }
    }

    pub fn add_pool_match(&mut self, mut pool_match: PoolMatch) {
        pool_match.id = self.max_id + 1;
        self.max_id += 1;
        self.pool_matches.push(pool_match);
    }
}

impl UserList {
    pub fn new() -> UserList {
        UserList {
            users: vec![],
        }
    }

    pub fn add_user(&mut self, user: String) -> usize {
        let user_id = self.users.len();
        self.users.push(user);
        user_id
    }
}
