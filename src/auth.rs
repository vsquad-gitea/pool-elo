use crate::user;
use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::{Deserialize, Serialize};

// References
// https://github.com/maxcountryman/axum-login/tree/main/examples/sqlite/src
// https://framesurge.sh/perseus/en-US/docs/0.4.x/state/intro

impl AuthUser for user::Model {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}
