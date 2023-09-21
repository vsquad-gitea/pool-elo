mod components;
mod templates;
mod data;
mod error_views;

use perseus::prelude::*;
use sycamore::prelude::view;

#[cfg(engine)]
use axum::{
    body::Body,
    extract::{Path, Query},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
#[cfg(engine)]
use perseus::turbine::ApiResponse as PerseusApiResponse;
#[cfg(engine)]
use perseus::{
    i18n::TranslationsManager,
    path::*,
    server::ServerOptions,
    stores::MutableStore,
    turbine::{SubsequentLoadQueryParams, Turbine},
};
#[cfg(engine)]
use tower_http::services::{ServeDir, ServeFile};



async fn print_something() {
    println!("haha");
}

#[cfg(engine)]
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) {
    use std::net::SocketAddr;

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    let mut app = perseus_axum::get_router(turbine, opts).await;
    app = app.route("/api/test", get(print_something));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[perseus::main(dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    env_logger::init();

    PerseusApp::new()
        .global_state_creator(crate::data::global_state::get_global_state_creator())
        .template(crate::templates::index::get_template())
        .template(crate::templates::add_game_form::get_template())
        .template(crate::templates::one_v_one_board::get_template())
        .template(crate::templates::overall_board::get_template())
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
