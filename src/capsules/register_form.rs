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
            models::auth::{RegisterRequest},
            endpoints::REGISTER,
            state_enums::OpenState,
            templates::get_api_path,
            global_state::AppStateRx,
            models::{
                generic::GenericResponse
            },
        };
        use reqwest::StatusCode;
    }
}

lazy_static! {
    pub static ref REGISTER_FORM: Capsule<PerseusNodeType, RegisterFormProps> = get_capsule();
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "RegisterFormStateRx")]
struct RegisterFormState {
    username: String,
    password: String,
    nickname: String,
    registration_code: String,
    email: String,
    error: String,
}

impl RegisterFormStateRx {
    #[cfg(client)]
    fn reset(&self) {
        self.username.set(String::new());
        self.password.set(String::new());
        self.nickname.set(String::new());
        self.registration_code.set(String::new());
        self.email.set(String::new());
        self.error.set(String::new());
    }
}

#[derive(Clone)]
pub struct RegisterFormProps {
    pub nickname: bool,
    pub registration_code: bool,
    pub email: bool,
}

#[auto_scope]
fn register_form_capsule<G: Html>(
    cx: Scope,
    state: &RegisterFormStateRx,
    props: RegisterFormProps,
) -> View<G> {
    let close_modal = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move {
                state.reset();
                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
                global_state.modals_open.register.set(OpenState::Closed)
            });
        }
    };

    let handle_register = move |_event: Event| {
        #[cfg(client)]
        {
            let registration_code = state.registration_code.get().as_ref().clone();
            spawn_local_scoped(cx, async move {
                let register_info = RegisterRequest {
                    username: state.username.get().as_ref().clone(),
                    password: state.password.get().as_ref().clone(),
                    nickname: state.nickname.get().as_ref().clone(),
                    email: state.email.get().as_ref().clone(),
                    registration_code,
                };

                // // @todo clean up error handling
                let client = reqwest::Client::new();
                let response = client
                    .post(get_api_path(REGISTER).as_str())
                    .json(&register_info)
                    .send()
                    .await
                    .unwrap();

                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
                let status = response.status();
                let response_data = response.json::<GenericResponse>().await.unwrap();
                if status != StatusCode::OK {
                    // todo update to some type of alert
                    state.error.set(response_data.status);
                    return;
                }

                // Update tentative username
                global_state
                    .auth
                    .username
                    .set(Some((*state.username.get()).clone()));

                // Open login modal
                global_state.modals_open.login.set(OpenState::Open);

                // Close modal
                state.reset();
                global_state.modals_open.register.set(OpenState::Closed);
            });
        }
    };

    view! { cx,
        dialog (class="modal-open modal modal-bottom sm:modal-middle"){
            div (class="modal-box") {
                // Header row - title and close button
                h3 (class="text-lg font-bold mb-4 text-center"){"Register"}
                button (on:click = close_modal, class = "btn btn-circle right-2 top-2 absolute") { CloseButtonSvg {} }

                // Add component for handling error messages
                ErrorBlock(error = state.error.clone())

                // Username field
                div (class = "label") { span (class = "label-text") { "Username" } }
                input (bind:value = state.username, class = "input input-bordered w-full")

                // Password field
                div (class = "label") { span (class = "label-text") { "Password" } }
                input (bind:value = state.password, type = "password", class = "input input-bordered w-full")

                (match props.registration_code {
                    true => { view! {cx,
                        div (class = "label") { span (class = "label-text") { "Registration Code" } }
                        input (bind:value = state.registration_code, class = "input input-bordered w-full")
                    }},
                    false => {view!{cx,}},
                })
                (match props.nickname {
                    true => { view! {cx,
                        div (class = "label") { span (class = "label-text") { "Nickname (Optional)" } }
                        input (bind:value = state.nickname, class = "input input-bordered w-full")
                    }},
                    false => {view!{cx,}},
                })
                (match props.email {
                    true => { view! {cx,
                        div (class = "label") { span (class = "label-text") { "Email (Optional)" } }
                        input (bind:value = state.email, class = "input input-bordered w-full")
                    }},
                    false => {view!{cx,}},
                })

                // Register button
                div (class = "flex justify-center mt-6") {
                    button (on:click = handle_register, class="btn"){"Register"}
                }
            }
        }
    }
}

pub fn get_capsule<G: Html>() -> Capsule<G, RegisterFormProps> {
    Capsule::build(Template::build("register_form").build_state_fn(get_build_state))
        .empty_fallback()
        .view_with_state(register_form_capsule)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> RegisterFormState {
    RegisterFormState {
        username: String::new(),
        password: String::new(),
        error: String::new(),
        nickname: String::new(),
        registration_code: String::new(),
        email: String::new(),
    }
}
