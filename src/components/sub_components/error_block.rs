use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::static_components::indicator::ErrorSvg;

#[component(inline_props)]
pub fn ErrorBlock<'a, G: Html>(cx: Scope<'a>, error: RcSignal<String>) -> View<G> {
    let error = create_ref(cx, error);
    let is_empty = create_selector(cx, || error.get().is_empty());

    view! { cx,
        (match !(*is_empty.get()) {
            true => { view!{cx,
                div (role="alert", class="alert alert-error") {
                    // Error icon
                    ErrorSvg {}

                    // Error text
                    span {(*error.get())}
                }
            }},
            false => {view!{cx,}},
        })
    }
}
