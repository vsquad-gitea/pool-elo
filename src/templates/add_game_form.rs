use crate::{components::layout::Layout, data::pool_match::MatchData};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use web_sys::Event;

cfg_if::cfg_if! {
    if #[cfg(client)] {
        use crate::data::pool_match::{PoolMatch, PoolMatchList};
        use crate::templates::global_state::AppStateRx;
        use crate::endpoints::MATCH;
        use crate::templates::get_api_path;
        use chrono::Utc;
    }
}

// Reactive page

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    name: String,
}

fn add_game_form_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a PageStateRx) -> View<G> {
    let handle_add_match = move |_event: Event| {
        #[cfg(client)]
        {
            // state.name.get().as_ref().clone()
            spawn_local_scoped(cx, async move {
                let new_match = PoolMatch::new(MatchData::Standard8Ball { winner: 1, loser: 2 }, Utc::now());
                let client = reqwest::Client::new();
                let new_matches = client
                    .post(get_api_path(MATCH).as_str())
                    .json(&new_match)
                    .send()
                    .await
                    .unwrap()
                    .json::<PoolMatchList>()
                    .await
                    .unwrap();
                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
                global_state.matches.set(new_matches);
            })
        }
    };

    view! { cx,
        Layout(title = "Add Game Results") {
            div (class = "flex flex-wrap") {
                input (bind:value = state.name,
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
        }
    }
}

#[engine_only_fn]
async fn get_request_state(
    _info: StateGeneratorInfo<()>,
    _req: Request,
) -> Result<PageState, BlamedError<std::convert::Infallible>> {
    Ok(PageState {
        name: "Ferris".to_string(),
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
