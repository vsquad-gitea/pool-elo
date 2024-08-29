use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use web_sys::Event;

cfg_if::cfg_if! {
    if #[cfg(client)] {
        use crate::{
            endpoints::LOGIN,
            global_state::{AppStateRx},
            models::auth::{LoginInfo, LoginResponse, WebAuthInfo},
            models::generic::GenericResponse,
            state_enums::{OpenState},
            templates::get_api_path,
        };
        use reqwest::StatusCode;
    }
}

lazy_static! {
    pub static ref LOGIN_FORM: Capsule<PerseusNodeType, LoginFormProps> = get_capsule();
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "LoginFormStateRx")]
struct LoginFormState {
    username: String,
    password: String,
    remember_me: bool,
    error: String,
}

impl LoginFormStateRx {
    #[cfg(client)]
    fn reset(&self) {
        self.username.set(String::new());
        self.password.set(String::new());
        self.remember_me.set(false);
        self.error.set(String::new());
    }
}

#[derive(Clone)]
pub struct LoginFormProps {
    pub remember_me: bool,
}

#[auto_scope]
fn login_form_capsule<G: Html>(
    cx: Scope,
    state: &LoginFormStateRx,
    props: LoginFormProps,
) -> View<G> {
    let close_modal = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move {
                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
                global_state.modals_open.login.set(OpenState::Closed)
            });
        }
    };

    let handle_forgot_password = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move {
                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
                global_state
                    .modals_open
                    .forgot_password
                    .set(OpenState::Open);
                // Close modal
                state.reset();
                global_state.modals_open.login.set(OpenState::Closed);
            });
        }
    };

    let handle_register = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move {
                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
                global_state.modals_open.register.set(OpenState::Open);
                // Close modal
                state.reset();
                global_state.modals_open.login.set(OpenState::Closed);
            });
        }
    };

    let handle_log_in = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move {
                let remember_me = *state.remember_me.get().as_ref();
                let username = state.username.get().as_ref().clone();
                let login_info = LoginInfo {
                    username: username.clone(),
                    password: state.password.get().as_ref().clone(),
                    remember_me,
                };

                // // @todo clean up error handling
                let client = reqwest::Client::new();
                let response = client
                    .post(get_api_path(LOGIN).as_str())
                    .json(&login_info)
                    .send()
                    .await
                    .unwrap();

                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);

                if response.status() != StatusCode::OK {
                    let response = response.json::<GenericResponse>().await.unwrap();
                    state.error.set(response.status.to_string());
                    return;
                }

                let response = response.json::<LoginResponse>().await.unwrap();

                // Save token to session/local storage and update state
                global_state.auth.handle_log_in(WebAuthInfo {
                    token: response.token,
                    expires: response.expires,
                    username,
                    remember_me,
                });

                // Close modal
                state.reset();
                global_state.modals_open.login.set(OpenState::Closed);
            });
        }
    };

    view! { cx,
        div (class="overflow-x-hidden overflow-y-auto fixed h-modal md:h-full top-4 left-0 right-0 md:inset-0 z-50 justify-center items-center"){
            div (class="relative md:mx-auto w-full md:w-1/2 lg:w-1/3 z-0 my-10") {
                div (class="bg-white rounded-lg shadow relative dark:bg-gray-700"){
                    div (class="flex justify-end p-2"){
                        button (on:click = close_modal, class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-800 dark:hover:text-white"){
                            "Close"
                        }
                    }
                    div (class="space-y-6 px-6 lg:px-8 pb-4 sm:pb-6 xl:pb-8") {
                        h3 (class="text-xl font-medium text-gray-900 dark:text-white"){"Sign in"}

                        (match state.error.get().as_ref() != "" {
                            true => { view!{cx,
                                div (role="alert") {
                                    div (class="bg-red-500 text-white font-bold rounded-t px-4 py-2") {
                                        "Error"
                                    }
                                    div (class="border border-t-0 border-red-400 rounded-b bg-red-100 px-4 py-3 text-red-700"){
                                        p {(state.error.get())}
                                    }
                                }
                            }},
                            false => {view!{cx,}},
                        })

                        div {
                            label (class="text-sm font-medium text-gray-900 block mb-2 dark:text-gray-300") {"Username"}
                            input (bind:value = state.username, class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white") {}
                        }
                        div {
                            label (class="text-sm font-medium text-gray-900 block mb-2 dark:text-gray-300"){"Password"}
                            input (bind:value = state.password, type = "password", class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white"){}
                        }
                        div (class="flex justify-between"){
                            (match props.remember_me {
                                true => { view!{ cx,
                                    div (class="flex items-start"){
                                        div (class="flex items-center h-5"){
                                            input (bind:checked = state.remember_me, type = "checkbox", class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600") {}
                                        }
                                        div (class="text-sm ml-3"){
                                            label (class="font-medium text-gray-900 dark:text-gray-300"){"Remember me"}
                                        }
                                    }
                                }},
                                false => view!{cx, },
                            })
                            button (on:click = handle_forgot_password, class="text-sm text-blue-700 hover:underline dark:text-blue-500"){"Lost Password?"}
                        }
                        button (on:click = handle_log_in, class="w-full text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"){"Log in"}
                        div (class="text-sm font-medium text-gray-500 dark:text-gray-300"){
                            button (on:click = handle_register, class="text-blue-700 hover:underline dark:text-blue-500"){"Create account"}
                        }
                    }
                }
            }
        }
    }
}

pub fn get_capsule<G: Html>() -> Capsule<G, LoginFormProps> {
    Capsule::build(Template::build("login_form").build_state_fn(get_build_state))
        .empty_fallback()
        .view_with_state(login_form_capsule)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> LoginFormState {
    LoginFormState {
        username: String::new(),
        password: String::new(),
        remember_me: false,
        error: String::new(),
    }
}
