mod capsules;
mod components;
mod endpoints;
#[allow(unused_imports)]
mod entity;
mod error_views;
mod global_state;
mod models;
#[cfg(engine)]
mod server;
mod state_enums;
mod templates;

use perseus::prelude::*;
use sycamore::prelude::view;

cfg_if::cfg_if! {
    if #[cfg(engine)] {
        use std::net::SocketAddr;
        use perseus::{
            i18n::TranslationsManager,
            server::ServerOptions,
            stores::MutableStore,
            turbine::Turbine,
        };
        use crate::server::routes::get_api_router;
        use crate::server::server_state::ServerState;
        use futures::executor::block_on;
        use sea_orm::Database;
    }
}

#[cfg(engine)]
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) {
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    let app = perseus_axum::get_router(turbine, opts).await;

    // TODO -> Update to use environment variable
    // TODO -> error handling
    // Includes making database connection
    let db_conn = Database::connect("postgres://elo:elo@localhost:5432/elo_app");
    let db_conn = block_on(db_conn);
    let db_conn = match db_conn {
        Ok(db_conn) => db_conn,
        Err(err) => {
            panic!("{}", err);
        }
    };
    let state = ServerState { db_conn };

    // Get server routes
    let api_router = get_api_router(state);
    let app = app.merge(api_router);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[perseus::main(dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    env_logger::init();

    PerseusApp::new()
        .global_state_creator(crate::global_state::get_global_state_creator())
        .template(crate::templates::index::get_template())
        .template(crate::templates::add_game_form::get_template())
        .template(crate::templates::one_v_one_board::get_template())
        .template(crate::templates::overall_board::get_template())
        .capsule_ref(&*crate::capsules::login_form::LOGIN_FORM)
        .capsule_ref(&*crate::capsules::forgot_password_form::FORGOT_PASSWORD_FORM)
        .error_views(crate::error_views::get_error_views())
        .index_view(|cx| {
            view! { cx,
                html (class = "flex w-full h-full"){
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        // Perseus automatically resolves `/.perseus/static/` URLs to the contents of the `static/` directory at the project root
                        link(rel = "stylesheet", href = ".perseus/static/style.css")
                    }
                    body (class = "w-full"){
                        // Quirk: this creates a wrapper `<div>` around the root `<div>` by necessity
                        PerseusRoot()
                    }
                }
            }
        })
}
