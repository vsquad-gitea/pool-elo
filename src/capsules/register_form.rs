use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use web_sys::Event;

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

                // Open login modal
                global_state.modals_open.login.set(OpenState::Open);
                state.reset();

                // Close modal
                state.reset();
                global_state.modals_open.register.set(OpenState::Closed);
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
                        h3 (class="text-xl font-medium text-gray-900 dark:text-white"){"Register"}


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
                        (match props.registration_code {
                            true => { view!{cx,
                                div {
                                    label (class="text-sm font-medium text-gray-900 block mb-2 dark:text-gray-300"){"Registration code"}
                                    input (bind:value = state.registration_code, class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white"){}
                                }
                            }},
                            false => {view!{cx,}},
                        })
                        (match props.nickname {
                            true => { view!{cx,
                                div {
                                    label (class="text-sm font-medium text-gray-900 block mb-2 dark:text-gray-300"){"Nickname (optional)"}
                                    input (bind:value = state.nickname, class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white"){}
                                }
                            }},
                            false => {view!{cx,}},
                        })
                        (match props.email {
                            true => { view!{cx,
                                div {
                                    label (class="text-sm font-medium text-gray-900 block mb-2 dark:text-gray-300"){"Email (optional)"}
                                    input (bind:value = state.email, class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white"){}
                                }
                            }},
                            false => {view!{cx,}},
                        })
                        button (on:click = handle_register, class="w-full text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"){"Register"}
                    }
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
