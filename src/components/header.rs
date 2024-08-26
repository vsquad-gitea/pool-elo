use std::sync::Arc;

use perseus::prelude::*;
use sycamore::prelude::*;
use web_sys::Event;

use crate::{
    capsules::login_form::{LoginFormProps, LOGIN_FORM},
    endpoints::LOGIN,
    models::auth::LoginInfo,
    state_enums::{GameState, LoginState, OpenState},
    templates::global_state::AppStateRx,
};

#[derive(Prop)]
pub struct HeaderProps<'a> {
    pub game: GameState,
    pub title: &'a str,
}

#[component]
pub fn Header<'a, G: Html>(cx: Scope<'a>, HeaderProps { game, title }: HeaderProps<'a>) -> View<G> {
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

    view! { cx,
        header {
            div (class = "flex items-center justify-between w-full md:text-center h-20") {
                div(class = "flex-1") {}

                // Title
                div(class = "text-gray-700 text-2xl font-semibold py-2") {
                    "Pool Elo - Season 1"
                }

                // Login / register or user buttons
                div(class = "flex-1 py-2") {(
                    match *global_state.auth.state.get() {
                        LoginState::NotAuthenticated => {
                            view! { cx,
                                button(class = "text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700") {
                                    "Register"
                                }
                                button(on:click = handle_log_in,class = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800") {
                                    "Login"
                                }
                            }
                        }
                        LoginState::Authenticated => {
                            view! { cx,
                                div {
                                    "Hello {username}!"
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

        section(class = "flex-2") {
            (match *global_state.modals_open.login.get() {
                OpenState::Open => {
                    view! { cx,
                        (LOGIN_FORM.widget(cx, "",
                            LoginFormProps{
                                remember_me: true,
                                endpoint: "".to_owned(),
                                lost_password_url: Some("".to_owned()),
                                forgot_password_url: Some("".to_owned()),
                            }
                        ))
                    }
                }
                OpenState::Closed => {
                    view!{ cx, }
                }
            })
        }
    }
}
