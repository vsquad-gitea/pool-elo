use perseus::prelude::*;
use sycamore::prelude::*;
use web_sys::Event;

use crate::components::static_components::close_button_path::CloseButtonPath;

#[component(inline_props)]
pub fn ErrorBlock<'a, G: Html>(cx: Scope<'a>, error: RcSignal<String>) -> View<G> {
    let error = create_ref(cx, error);
    let is_empty = create_selector(cx, || error.get().is_empty());

    let close_block = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move { error.set(String::new()) });
        }
    };

    view! { cx,
        (match !(*is_empty.get()) {
            true => { view!{cx,

                div (class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative", role="alert") {
                    span (class="block sm:inline"){
                        p {(*error.get())}
                    }
                    span (class="absolute top-0 bottom-0 right-0 px-4 py-3"){
                        svg (on:click = close_block, class="fill-current h-6 w-6 text-red-500", role="button", viewBox="0 0 20 20") {
                            title {"Close"}
                            CloseButtonPath {}
                        }
                    }
                }
            }},
            false => {view!{cx,}},
        })
    }
}
