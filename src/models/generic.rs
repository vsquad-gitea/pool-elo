use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GenericResponse {
    pub status: String,
}

impl GenericResponse {
    pub fn ok() -> Self {
        GenericResponse {
            status: String::new(),
        }
    }
    pub fn err(msg: &str) -> Self {
        GenericResponse {
            status: msg.to_string(),
        }
    }
}
