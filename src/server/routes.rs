// (Server only) Routes
use crate::endpoints::{LOGIN, LOGIN_TEST};
use axum::routing::{post, Router};

use super::auth::login::{post_login_user, post_test_login};

pub fn register_routes(app: Router) -> Router {
    let app = app.route(LOGIN, post(post_login_user));
    let app = app.route(LOGIN_TEST, post(post_test_login));
    app
}
