use crate::{components::layout::Layout, templates::global_state::AppStateRx, data::user::PlayerId};

use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::data::pool_match::PoolMatch;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {}

fn format_list_or_single(to_format: &Vec<PlayerId>) -> String{
    match to_format.len() {
        1 => to_format[0].to_string(),
        _ => format!("{:?}", to_format),
    }
}

fn overall_board_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, _state: &'a PageStateRx) -> View<G> {
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);

    view! { cx,
        Layout(title = "Overall Leaderboard") {
            ul {
                (View::new_fragment(
                    global_state.matches.get()
                        .pool_matches
                        .iter()
                        .rev()
                        .map(|item: &PoolMatch| {
                            let game = item.clone();
                            
                            view! { cx,
                                li (class = "text-blue-700", id = "ha",) {
                                    (game.id)
                                    ("  ")
                                    (format_list_or_single(&game.data.winners))
                                    ("  ")
                                    (format_list_or_single(&game.data.losers))
                                }
                            }
                        })
                        .collect(),
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
    Template::build("overall-board")
        .request_state_fn(get_request_state)
        .view_with_state(overall_board_page)
        .head(head)
        .build()
}
