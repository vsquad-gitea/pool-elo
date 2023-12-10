// (Server only) In-memory data storage and persistent storage

use crate::data::pool_match::PoolMatchList;
use crate::data::pool_match::UserList;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, sync::Mutex};

#[derive(Serialize, Deserialize, Clone)]
pub struct Store {
    pub matches: PoolMatchList,
    pub users: UserList,
}

impl Store {
    fn new() -> Store {
        fs::create_dir_all("data").unwrap();
        match Path::new("data/store.json").exists() {
            false => Store {
                matches: PoolMatchList::new(),
                users: UserList::new(),
            },
            true => {
                let contents = fs::read_to_string("data/store.json").unwrap();
                serde_json::from_str(&contents).unwrap()
            }
        }
    }
    // TODO -> Store data
    #[allow(dead_code)]
    pub fn write(&self) {
        let contents = serde_json::to_string(&self).unwrap();
        fs::write("data/store.json", contents).unwrap();
    }
}

pub static DATA: Lazy<Mutex<Store>> = Lazy::new(|| Mutex::new(Store::new()));
