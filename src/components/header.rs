use perseus::prelude::*;
use sycamore::prelude::*;
use web_sys::Event;

use crate::{
    components::static_components::menu_button::MenuButtonSvg,
    global_state::AppStateRx,
    state_enums::{ContentState, LoginState},
};

cfg_if::cfg_if! {
    if #[cfg(client)] {
        use crate::{
            state_enums::OpenState,
        };
    }
}

#[derive(Prop)]
pub struct HeaderProps {
    pub content_state: ContentState,
}

// TODO update to have user preferences
#[component]
fn LinkList<G: Html>(cx: Scope) -> View<G> {
    // Get global state to get style info
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);

    view! { cx,
        li {
            a (href = "pool") {
                label (class = "swap") {
                    input (
                        type="radio",
                        name = "theme-dropdown",
                        class = "theme-controller",
                        value = (*global_state.style.theme.pool.get()),
                    ) {}
                    p (class = (
                        if *global_state.style.theme.current.get() == *global_state.style.theme.pool.get(){
                            "font-bold"
                        }
                        else {
                            ""
                        }
                    )){ "Pool" }
                }
            }
        }
        li {
            a (href = "table_tennis") {
                label (class = "swap") {
                    input (
                        type="radio",
                        name = "theme-dropdown",
                        class = "theme-controller",
                        value = (*global_state.style.theme.table_tennis.get()),
                    ) {}
                    p (class = (
                        if *global_state.style.theme.current.get() == *global_state.style.theme.table_tennis.get(){
                            "font-bold"
                        }
                        else {
                            ""
                        }
                    )){ "Table Tennis" }
                }
            }
        }
        li {
            a (href = "pickleball") {
                label (class = "swap") {
                    input (
                        type="radio",
                        name = "theme-dropdown",
                        class = "theme-controller",
                        value = (*global_state.style.theme.pickleball.get()),
                    ) {}
                    p (class = (
                        if *global_state.style.theme.current.get() == *global_state.style.theme.pickleball.get(){
                            "font-bold"
                        }
                        else {
                            ""
                        }
                    )){ "Pickleball" }
                }
            }
        }
    }
}

#[component]
pub fn Header<G: Html>(cx: Scope, props: HeaderProps) -> View<G> {
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);

    let handle_log_in = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move {
                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
                global_state.modals_open.login.set(OpenState::Open);
            });
        }
    };

    let handle_register = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move {
                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
                global_state.modals_open.register.set(OpenState::Open);
            });
        }
    };

    let handle_log_out = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move {
                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
                global_state.auth.handle_log_out();
            });
        }
    };

    view! { cx,
        header (class="navbar bg-base-100") {
            // Navigation
            div (class="navbar-start") {
                div (class="dropdown") {
                    div (tabindex="0", role="button", class="btn btn-ghost lg:hidden") { MenuButtonSvg {} }
                    ul (tabindex = "0", class = "menu menu-sm dropdown-content bg-base-100 rounded-box z-[1] mt-3 w-52 p-2 shadow" ) {
                        LinkList {}
                    }
                }
                ul (class="menu menu-horizontal px-1 hidden lg:flex") {
                    LinkList {}
                }
            }
            // Title
            div (class="navbar-center lg:flex") {
                (props.content_state.to_string())
            }
            // User buttons
            div (class="navbar-end") {
                (match *global_state.auth.state.get() {
                    LoginState::Authenticated => { view! { cx,
                        button(on:click = handle_log_out, class = "btn btn-primary mr-2") {
                            "Log out"
                        }
                    } },
                    LoginState::NotAuthenticated => { view! { cx,
                        button(on:click = handle_register, class = "btn btn-primary mr-2") {
                            "Register"
                        }
                        button(on:click = handle_log_in, class = "btn btn-secondary mr-2") {
                            "Log in"
                        }
                    } },
                    LoginState::Unknown => { view! { cx,
                        div (class = "px-5 py-2.5 me-2 mb-2") {
                            "Loading..."
                        }
                    } },
                })
            }
        }
    }
}
