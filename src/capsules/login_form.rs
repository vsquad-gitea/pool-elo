use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use web_sys::Event;

use crate::components::{
    static_components::close_button::CloseButtonSvg, sub_components::error_block::ErrorBlock,
};

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
                state.reset();
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
        dialog (class="modal-open modal modal-bottom sm:modal-middle") {
            div (class="modal-box"){
                // Header row - title and close button
                h3 (class="text-lg font-bold mb-4 text-center"){"Sign in"}
                button (on:click = close_modal, class = "btn btn-circle right-2 top-2 absolute") { CloseButtonSvg {} }

                // Add component for handling error messages
                ErrorBlock(error = state.error.clone())

                // Username field
                div (class = "label") { span (class = "label-text") { "Username" } }
                input (bind:value = state.username, class = "input input-bordered w-full")

                // Password field
                div (class = "label") { span (class = "label-text") { "Password" } }
                input (bind:value = state.password, type = "password", class = "input input-bordered w-full")

                // Remember me button and forget password button
                div (class="flex justify-between items-center mt-1"){
                    // Remember me button
                    (match props.remember_me {
                        true => { view!{ cx,
                            div (class = "flex items-start  form-control") {
                                label (class = "label cursor-pointer") {
                                    span (class = "label-text mr-4") { "Remember me" }
                                    input (bind:checked = state.remember_me, type = "checkbox", class = "checkbox")
                                }
                            }
                        }},
                        false => view!{cx, },
                    })
                    // Forget password button
                    button (on:click = handle_forgot_password, class="flex  link link-primary"){"Lost Password?"}
                }

                // Log in button
                div (class = "flex justify-center") {
                    button (on:click = handle_log_in, class="btn"){"Log in"}
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
