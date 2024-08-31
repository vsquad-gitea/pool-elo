use crate::{components::layout::Layout, global_state::AppStateRx, state_enums::ContentState};

use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {}

fn overall_board_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, _state: &'a PageStateRx) -> View<G> {
    let _global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);

    view! { cx,
        Layout(content_state = ContentState::Pool) {
            ul {
                (View::new_fragment(
                    vec![],
                ))
            }
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
        title { "Overall leaderboard" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("pool/overall-board")
        .request_state_fn(get_request_state)
        .view_with_state(overall_board_page)
        .head(head)
        .build()
}
