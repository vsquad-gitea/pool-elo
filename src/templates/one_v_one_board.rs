use crate::{components::layout::Layout, state_enums::GameState};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {}

fn one_v_one_board_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, _state: &'a PageStateRx) -> View<G> {
    view! { cx,
        Layout(game = GameState::Pool) {
            p { "leaderboard" }
        }
    }
}

#[engine_only_fn]
async fn get_request_state(
    _info: StateGeneratorInfo<()>,
    _req: Request,
) -> Result<PageState, BlamedError<std::convert::Infallible>> {
    Ok(PageState {})
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "1v1 Leaderboard" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("one-v-one-board")
        .request_state_fn(get_request_state)
        .view_with_state(one_v_one_board_page)
        .head(head)
        .build()
}
