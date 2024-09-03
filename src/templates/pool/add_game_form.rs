use crate::{
    components::{layout::Layout, sub_components::error_block::ErrorBlock},
    state_enums::ContentState,
};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use web_sys::Event;

cfg_if::cfg_if! {
    if #[cfg(client)] {
        use crate::{
            endpoints::ADD_MATCH,
            models::{game_result::GameResult, generic::GenericResponse},
            templates::get_api_path,
        };

        use crate::entity::sea_orm_active_enums::GameType;
        use reqwest::StatusCode;
    }
}

// Reactive page

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    winner: String,
    loser: String,
    error: String,
}

fn add_game_form_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a PageStateRx) -> View<G> {
    let handle_add_match = move |_event: Event| {
        #[cfg(client)]
        {
            let var_name = async move {
                let game_result = GameResult {
                    winner: (*state.winner.get()).clone(),
                    loser: (*state.loser.get()).clone(),
                    game_type: GameType::Pool,
                };

                let client = reqwest::Client::new();
                let response = client
                    .post(get_api_path(ADD_MATCH).as_str())
                    .json(&game_result)
                    .send()
                    .await
                    .unwrap();

                if response.status() != StatusCode::OK {
                    let response = response.json::<GenericResponse>().await.unwrap();
                    state.error.set(response.status.to_string());
                    return;
                }

                // let response = response.json::<())>().await.unwrap();
            };
            let var_name = var_name;
            spawn_local_scoped(cx, var_name)
        }
    };

    view! { cx,
        Layout(content_state = ContentState::Pool) {

            ErrorBlock(error = state.error.clone())

            div (class = "label") { span (class = "label-text") { "Winner" } }
            input (bind:value = state.winner, class = "input input-bordered w-full m-2")

            div (class = "label") { span (class = "label-text") { "Loser" } }
            input (bind:value = state.loser, class = "input input-bordered w-full m-2")

            div (class = "flex justify-center") {
                button (on:click = handle_add_match, class="btn"){"Record Match"}
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
        winner: String::new(),
        loser: String::new(),
        error: String::new(),
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
    Template::build("pool/add-game-form")
        .request_state_fn(get_request_state)
        .view_with_state(add_game_form_page)
        .head(head)
        .build()
}
