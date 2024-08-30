use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use web_sys::Event;

use crate::{
    components::{
        static_components::close_button::CloseButtonSvg, sub_components::error_block::ErrorBlock,
    },
    global_state::AppStateRx,
};

cfg_if::cfg_if! {
    if #[cfg(client)] {
        use crate::{
            state_enums::{ OpenState},
            templates::get_api_path,
            endpoints::FORGOT_PASSWORD,
            models::{
                auth::ForgotPasswordRequest,
                generic::GenericResponse,
            },
        };
        use reqwest::StatusCode;

    }
}

lazy_static! {
    pub static ref FORGOT_PASSWORD_FORM: Capsule<PerseusNodeType, ForgotPasswordFormProps> =
        get_capsule();
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "ForgotPasswordFormStateRx")]
struct ForgotPasswordFormState {
    username: String,
    how_to_reach: String,
    error: String,
}

impl ForgotPasswordFormStateRx {
    #[cfg(client)]
    fn reset(&self) {
        self.username.set(String::new());
        self.how_to_reach.set(String::new());
        self.error.set(String::new());
    }
}

#[derive(Clone)]
pub struct ForgotPasswordFormProps {}

#[auto_scope]
fn forgot_password_form_capsule<G: Html>(
    cx: Scope,
    state: &ForgotPasswordFormStateRx,
    _props: ForgotPasswordFormProps,
) -> View<G> {
    // If there's a tentative username, set it
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
    if let Some(username) = (*global_state.auth.username.get()).clone() {
        state.username.set(username);
        global_state.auth.username.set(None);
    }

    let close_modal = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move {
                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
                // Close modal
                state.reset();
                global_state
                    .modals_open
                    .forgot_password
                    .set(OpenState::Closed)
            });
        }
    };
    let handle_submit = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move {
                let request = ForgotPasswordRequest {
                    username: state.username.get().as_ref().clone(),
                    contact_info: state.how_to_reach.get().as_ref().clone(),
                };

                // // @todo clean up error handling
                let client = reqwest::Client::new();
                let response = client
                    .post(get_api_path(FORGOT_PASSWORD).as_str())
                    .json(&request)
                    .send()
                    .await
                    .unwrap();
                let status = response.status();
                let response_data = response.json::<GenericResponse>().await.unwrap();
                if status != StatusCode::OK {
                    state.error.set(response_data.status);
                    return;
                }

                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);

                // Close modal
                state.reset();
                global_state
                    .modals_open
                    .forgot_password
                    .set(OpenState::Closed);
            });
        }
    };

    view! { cx,
        dialog (class="modal-open modal modal-bottom sm:modal-middle animate-none") {
            div (class="modal-box"){
                // Header row - title and close button
                h3 (class="text-lg font-bold mb-4 text-center"){"Forgot Password"}
                button (on:click = close_modal, class = "btn btn-circle right-2 top-2 absolute") { CloseButtonSvg {} }

                // Add component for handling error messages
                ErrorBlock(error = state.error.clone())

                // Username field
                div (class = "label") { span (class = "label-text") { "Username" } }
                input (bind:value = state.username, class = "input input-bordered w-full")

                // Password field
                div (class = "label") { span (class = "label-text") { "Contact Info" } }
                input (bind:value = state.how_to_reach, class = "input input-bordered w-full")

                // Submit button
                div (class = "flex justify-center") {
                    button (on:click = handle_submit, class="btn"){"Submit"}
                }
            }
        }
    }
}

pub fn get_capsule<G: Html>() -> Capsule<G, ForgotPasswordFormProps> {
    Capsule::build(Template::build("forgot_password_form").build_state_fn(get_build_state))
        .empty_fallback()
        .view_with_state(forgot_password_form_capsule)
        .build()
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> ForgotPasswordFormState {
    ForgotPasswordFormState {
        username: String::new(),
        how_to_reach: String::new(),
        error: String::new(),
    }
}
