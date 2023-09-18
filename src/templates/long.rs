use crate::components::layout::Layout;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use sycamore::prelude::*;
use crate::templates::get_path;

// Reactive page

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    ip: String,
    text: String,
}

fn request_state_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a PageStateRx) -> View<G> {
    view! { cx,
        p {
            (
                format!("Your IP address is {}.", state.ip.get())
            )
        }
        p {
            (
                state.text.get().repeat(5000)
            )
        }
    }
}

#[engine_only_fn]
async fn get_request_state(
    _info: StateGeneratorInfo<()>,
    req: Request,
) -> Result<PageState, BlamedError<std::convert::Infallible>> {
    let text = fs::read_to_string("static/test.txt")
        .unwrap_or("~".to_string())
        .parse()
        .unwrap();

    Ok(PageState {
        ip: format!(
            "{:?}",
            req.headers()
                // NOTE: This header can be trivially spoofed, and may well not be the user's actual
                // IP address
                .get("X-Forwarded-For")
                .unwrap_or(&perseus::http::HeaderValue::from_str("hidden from view!").unwrap())
        ),
        text,
    })
}

// Regular page

fn long_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Layout(title = "Long") {
            // Anything we put in here will be rendered inside the `<main>` block of the layout
            a(href = "") { "Index" }
            br {}
            p {
                ("This is a test. ".repeat(5000))
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Long Page" }
    }
}

// Template

pub fn get_template<G: Html>() -> Template<G> {
    // Template::build(get_full_path("long")).view(long_page).head(head).build()
    Template::build(get_path("long").as_str())
        .request_state_fn(get_request_state)
        .view_with_state(request_state_page)
        .head(head)
        .build()
}
