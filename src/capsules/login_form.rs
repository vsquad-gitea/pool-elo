use lazy_static::lazy_static;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

lazy_static! {
    pub static ref LOGIN_FORM: Capsule<PerseusNodeType, LoginFormProps> = get_capsule();
}

#[auto_scope]
fn login_form_capsule<G: Html>(
    cx: Scope,
    state: &LoginFormStateRx,
    props: LoginFormProps,
) -> View<G> {
    view! {
        cx,
        div (class="overflow-x-hidden overflow-y-auto fixed h-modal md:h-full top-4 left-0 right-0 md:inset-0 z-50 justify-center items-center"){
            div (class="relative w-full max-w-md px-4 h-full md:h-auto") {
                div (class="bg-white rounded-lg shadow relative dark:bg-gray-700"){
                    div (class="flex justify-end p-2"){
                        button (class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-800 dark:hover:text-white"){
                            "Back"
                        }
                    }
                    form (class="space-y-6 px-6 lg:px-8 pb-4 sm:pb-6 xl:pb-8") {
                        h3 (class="text-xl font-medium text-gray-900 dark:text-white"){"Sign in to our platform"}
                        div {
                            label (class="text-sm font-medium text-gray-900 block mb-2 dark:text-gray-300") {"Your email"}
                            input (class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white") {}
                        }
                        div {
                            label (class="text-sm font-medium text-gray-900 block mb-2 dark:text-gray-300"){"Your password"}
                            input (class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white"){}
                        }
                        div (class="flex justify-between"){
                            div (class="flex items-start"){
                                div (class="flex items-center h-5"){
                                    input (class="bg-gray-50 border border-gray-300 focus:ring-3 focus:ring-blue-300 h-4 w-4 rounded dark:bg-gray-600 dark:border-gray-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800") {}
                                }
                                div (class="text-sm ml-3"){
                                label (class="font-medium text-gray-900 dark:text-gray-300"){"Remember me"}
                                }
                            }
                            a (class="text-sm text-blue-700 hover:underline dark:text-blue-500"){"Lost Password?"}
                        }
                        button (class="w-full text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"){"Login to your account"}
                        div (class="text-sm font-medium text-gray-500 dark:text-gray-300"){
                            a (class="text-blue-700 hover:underline dark:text-blue-500"){"Create account"}
                        }
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "LoginFormStateRx")]
struct LoginFormState {
    username: String,
    password: String,
}

#[derive(Clone)]
pub struct LoginFormProps {
    pub remember_me: bool,
    pub endpoint: String,
    pub lost_password_url: Option<String>,
    pub forgot_password_url: Option<String>,
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
        username: "".to_string(),
        password: "".to_string(),
    }
}
