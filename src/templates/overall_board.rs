use crate::components::layout::Layout;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use sycamore::prelude::*;

// Reactive page

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {

}

fn overall_board_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a PageStateRx) -> View<G> {
    view! { cx,
        Layout(title = "Overall Leaderboard") {
            // Anything we put in here will be rendered inside the `<main>` block of the layout
            p { "leaderboard" }
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
        title { "Overall leaderboard" }
    }
}


// Template

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("overall-board")
        .request_state_fn(get_request_state)
        .view_with_state(overall_board_page)
        .head(head)
        .build()
}
