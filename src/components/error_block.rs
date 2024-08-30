use perseus::prelude::*;
use sycamore::prelude::*;
use web_sys::Event;

cfg_if::cfg_if! {
    if #[cfg(client)] {
        use crate::{
            state_enums::OpenState,
        };
    }
}

#[derive(Prop, Debug, Clone)]
pub struct ErrorBlockProps {
    error: RcSignal<String>,
}

#[component]
pub fn ErrorBlock<G: Html>(cx: Scope, props: ErrorBlockProps) -> View<G> {
    let is_empty = create_selector(cx, || props.error.get().is_empty());
    view! { cx,
        (match *is_empty.get() {
            true => { view!{cx,
                div (role="alert") {
                    div (class="bg-red-500 text-white font-bold rounded-t px-4 py-2") {
                        "Error"
                    }
                    div (class="border border-t-0 border-red-400 rounded-b bg-red-100 px-4 py-3 text-red-700"){
                        p {(*props.error.get())}
                    }
                }
            }},
            false => {view!{cx,}},
        })
    }
}
