use crate::{components::layout::Layout, state_enums::GameState};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use web_sys::Event;

cfg_if::cfg_if! {
    if #[cfg(client)] {
        use crate::templates::global_state::AppStateRx;
        use crate::templates::get_api_path;
        use chrono::Utc;
    }
}

// Reactive page

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    winner: String,
    new_user: String,
}

fn add_game_form_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a PageStateRx) -> View<G> {
    let handle_add_match = move |_event: Event| {
        #[cfg(client)]
        {
            // state.winner.get().as_ref().clone()
            spawn_local_scoped(cx, async move {})
        }
    };

    let handle_add_user = move |_event: Event| {
        #[cfg(client)]
        {
            // state.winner.get().as_ref().clone()
            spawn_local_scoped(cx, async move {})
        }
    };

    view! { cx,
        Layout(title = "Add Game Results", game = GameState::Pool) {
            div (class = "flex flex-wrap") {
                select {
                    option (value="red")
                    option (value="blue")
                }
            }
            div (class = "flex flex-wrap") {
                input (bind:value = state.winner,
                       class = "appearance-none block w-full bg-gray-200 text-gray-700 border \
                       border-red-500 rounded py-3 px-4 mb-3 leading-tight focus:outline-none \
                       focus:bg-white",)
            }
            div (class = "flex flex-wrap") {
                button(on:click = handle_add_match,
                       class = "flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-teal-500 \
                       hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded",
                ) {
                    "Add result"
                }
            }
            div (class = "flex flex-wrap") {
                input (bind:value = state.new_user,
                       class = "appearance-none block w-full bg-gray-200 text-gray-700 border \
                       border-red-500 rounded py-3 px-4 mb-3 leading-tight focus:outline-none \
                       focus:bg-white",)
            }
            div (class = "flex flex-wrap") {
                button(on:click = handle_add_user,
                       class = "flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-teal-500 \
                       hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded",
                ) {
                    "Add new user"
                }
            }
        }
    }
}

#[engine_only_fn]
async fn get_request_state(
    _info: StateGeneratorInfo<()>,
    _req: Request,
) -> Result<PageState, BlamedError<std::convert::Infallible>> {
    Ok(PageState {
        winner: "Ferris".to_owned(),
        new_user: "newguy".to_owned(),
    })
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
