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
        use futures::executor::block_on;
        use sea_orm::{Database};
        use crate::server::routes::register_routes;
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
    let mut app = perseus_axum::get_router(turbine, opts).await;

    app = register_routes(app);

    // TODO -> Update to use environment variable
    if let Err(err) = block_on(Database::connect(
        "postgres://elo:elo@localhost:5432/elo_app",
    )) {
        panic!("{}", err);
    }

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
