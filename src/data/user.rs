use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::{Deserialize, Serialize};

pub type PlayerId = u32;

// References
// https://github.com/maxcountryman/axum-login/tree/main/examples/sqlite/src
// https://framesurge.sh/perseus/en-US/docs/0.4.x/state/intro

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    id: u64,
    pub username: String,
    password: String,
}

// Override debug to prevent logging password hash
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[hidden]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = u64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}
