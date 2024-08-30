use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn CloseButtonSvg<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        svg (
            xmlns = "http://www.w3.org/2000/svg",
            class = "h-6 w-6 stroke-primary",
            fill = "none",
            viewBox = "0 0 24 24",
        ) {
            path (
               stroke-linecap = "round",
               stroke-linejoin = "round",
               stroke-width = "2",
               d = "M6 18L18 6M6 6l12 12"
            ){}
        }
    }
}

#[component]
pub fn CloseButtonPath<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        path (d="M14.348 14.849a1.2 1.2 0 0 1-1.697 0L10 11.819l-2.651 3.029a1.2 1.2 0 1 1-1.697-1.697l2.758-3.15-2.759-3.152a1.2 1.2 0 1 1 1.697-1.697L10 8.183l2.651-3.031a1.2 1.2 0 1 1 1.697 1.697l-2.758 3.152 2.758 3.15a1.2 1.2 0 0 1 0 1.698z"){}
    }
}
