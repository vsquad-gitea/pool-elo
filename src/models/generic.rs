use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GenericResponse {
    pub status: String,
}

impl GenericResponse {
    #[cfg(engine)]
    pub fn ok() -> Self {
        GenericResponse {
            status: String::new(),
        }
    }
    #[cfg(engine)]
    pub fn err(msg: &str) -> Self {
        GenericResponse {
            status: msg.to_string(),
        }
    }
}
