use crate::components::layout::Layout;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(engine)]
use crate::data::store::DATA;
#[cfg(engine)]
use std::thread;
use sycamore::prelude::*;

use crate::data::pool_match::{
    PoolMatchList, PoolMatch
};

// Reactive page

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "PageStateRx")]
struct PageState {
    matches: PoolMatchList,
}

fn overall_board_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a PageStateRx) -> View<G> {
    view! { cx,
        Layout(title = "Overall Leaderboard") {
            // Anything we put in here will be rendered inside the `<main>` block of the layout
            ul {
                (View::new_fragment(
                    state.matches.get()
                        .pool_matches
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(index, item)| {
                            let game = item.clone();
                            view! { cx,
                                li {
                                    (game.winner)
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
    req: Request,
) -> Result<PageState, BlamedError<std::convert::Infallible>> {

    let matches = thread::spawn(move || {
        let mut db = DATA.lock().unwrap();
        db.matches.pool_matches.push(PoolMatch {
            players: vec![],
            winner: "lol".to_string(),
        });
        db.write();
        db.matches.clone()
    }).join().unwrap();

    Ok(PageState {
        matches
    })
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
