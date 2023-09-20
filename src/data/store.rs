#![cfg(engine)]

use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use crate::data::pool_match::{PoolMatchList, PoolMatch};
use std::fs;
use std::path::Path;


#[derive(Serialize, Deserialize, Clone)]
pub struct Store {
    pub matches: PoolMatchList,
}

impl Store {
    fn new() -> Store {
        fs::create_dir_all("data");
        match Path::new("data/store.json").exists() {
            false => {
                Store {
                    matches: PoolMatchList { pool_matches: vec![] },
                }
            }
            true => {
                let contents = fs::read_to_string("data/store.json").unwrap();
                serde_json::from_str(&contents).unwrap()
            }
        }
    }
    pub fn write(&self) {
        let contents = serde_json::to_string(&self).unwrap();
        fs::write("data/store.json", contents).unwrap();
    }
}

pub static DATA: Lazy<Mutex<Store>> = Lazy::new(|| {
    Mutex::new(Store::new())
});
