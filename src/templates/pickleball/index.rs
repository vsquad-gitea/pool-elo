use crate::{components::layout::Layout, state_enums::ContentState};
use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Layout(content_state = ContentState::Pickleball) {
            // Anything we put in here will be rendered inside the `<main>` block of the layout
            p { "Hello World!" }
            br {}
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("pickleball")
        .view(index_page)
        .head(head)
        .build()
}
