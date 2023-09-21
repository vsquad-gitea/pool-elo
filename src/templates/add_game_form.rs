use std::ops::Deref;
use crate::components::layout::Layout;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use crate::data::global_state::AppStateRx;
use web_sys::{window, Event};
use crate::data::pool_match::PoolMatch;
#[cfg(client)]
use perseus::utils::get_path_prefix_client;
use crate::templates::get_api_path;


// Reactive page

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {}



fn add_game_form_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a PageStateRx) -> View<G> {
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
    let api_path = get_api_path("/api/test");

    let handle_add_match = move |event: Event| {
        #[cfg(client)]
        {
            let path = get_api_path("/api/test");
            println!("{}", path);
            spawn_local_scoped(cx, async move {
                reqwest::get(get_api_path("/api/test").as_str()).await.unwrap();
            })
        }
    };

    view! { cx,
        Layout(title = "Add Game Results") {
            // Anything we put in here will be rendered inside the `<main>` block of the layout
            button(on:click=handle_add_match) {
                "Add result"
            }
            p {
                (api_path)
            }
        }
    }
}

#[engine_only_fn]
async fn get_request_state(
    _info: StateGeneratorInfo<()>,
    req: Request,
) -> Result<PageState, BlamedError<std::convert::Infallible>> {
    Ok(PageState {})
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Add Game Form" }
    }
}

// Template

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("add-game-form")
        .request_state_fn(get_request_state)
        .view_with_state(add_game_form_page)
        .head(head)
        .build()
}
