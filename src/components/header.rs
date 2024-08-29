use perseus::prelude::*;
use sycamore::prelude::*;
use web_sys::Event;

use crate::{
    global_state::AppStateRx,
    state_enums::{GameState, LoginState},
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
    pub game: GameState,
}

#[component]
pub fn Header<G: Html>(cx: Scope, props: HeaderProps) -> View<G> {
    // Get global state to get authentication info
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
        header {
            div (class = "flex items-center justify-between w-full md:text-center h-20") {
                div(class = "flex-1") {}

                // Title
                div(class = "text-gray-700 text-2xl font-semibold py-2") {
                   (props.game.to_string()) " - Season 1"
                }

                // Login / register or user buttons
                div(class = "flex-1 py-2") {(
                    match *global_state.auth.state.get() {
                        LoginState::NotAuthenticated => {
                            view! { cx,
                                button(on:click = handle_register, class = "text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700") {
                                    "Register"
                                }
                                button(on:click = handle_log_in, class = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800") {
                                    "Log in"
                                }
                            }
                        }
                        LoginState::Authenticated => {
                            view! { cx,
                                div {
                                    "Hello "
                                        (global_state.auth.username.get().as_ref().clone().unwrap_or("".to_owned()))
                                }
                                button(on:click = handle_log_out, class = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800") {
                                    "Log out"
                                }
                            }
                        }
                        // Will only appear for a few seconds
                        LoginState::Unknown => {
                            view! { cx,
                                div (class = "px-5 py-2.5 me-2 mb-2"){
                                    "Loading..."
                                }
                            }
                        },
                    })
                }
            }
        }
    }
}
