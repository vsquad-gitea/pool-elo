use crate::data::user::PlayerId;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type MatchId = u32;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MatchData {
    Standard8Ball {
        winner: PlayerId,
        loser: PlayerId,
    },
    Standard9Ball {
        winner: PlayerId,
        loser: PlayerId,
    },
    CutThroat {
        winner: PlayerId,
        losers: [PlayerId; 2],
    },
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
