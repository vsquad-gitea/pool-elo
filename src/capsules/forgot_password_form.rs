use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use web_sys::Event;

use crate::components::sub_components::error_block::ErrorBlock;

cfg_if::cfg_if! {
    if #[cfg(client)] {
        use crate::{
            state_enums::{ OpenState},
            templates::get_api_path,
            global_state::{AppStateRx},
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
        div (class="overflow-x-hidden overflow-y-auto fixed h-modal md:h-full top-4 left-0 right-0 md:inset-0 z-50 justify-center items-center"){
            div (class="relative md:mx-auto w-full md:w-1/2 lg:w-1/3 z-0 my-10") {
                div (class="bg-white rounded-lg shadow relative dark:bg-gray-700"){
                    div (class="flex justify-end p-2"){
                        button (on:click = close_modal, class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-800 dark:hover:text-white"){
                            "Close"
                        }
                    }
                    div (class="space-y-6 px-6 lg:px-8 pb-4 sm:pb-6 xl:pb-8") {
                        h3 (class="text-xl font-medium text-gray-900 dark:text-white"){"Forgot Password"}

                        // Add component for handling error messages
                        ErrorBlock(error = state.error.clone())

                        div {
                            label (class="text-sm font-medium text-gray-900 block mb-2 dark:text-gray-300") {"Username"}
                            input (bind:value = state.username, class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white") {}
                        }
                        div {
                            label (class="text-sm font-medium text-gray-900 block mb-2 dark:text-gray-300"){"Contact Info"}
                            input (bind:value = state.how_to_reach, class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white"){}
                        }

                        button (on:click = handle_submit, class="w-full text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"){"Submit"}
                    }
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
