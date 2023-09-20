mod components;
mod templates;
mod data;
mod error_views;

use perseus::prelude::*;
use sycamore::prelude::view;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    env_logger::init();

    PerseusApp::new()
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
