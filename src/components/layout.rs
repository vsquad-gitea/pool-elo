use crate::templates::global_state::{AppStateRx, LoginState};
use perseus::prelude::*;
use sycamore::prelude::*;
use web_sys::Event;

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    pub _title: &'a str,
    pub children: Children<'a, G>,
}

// Using elements from here: https://flowbite.com/docs/components/buttons/

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps {
        _title: _,
        children,
    }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);
    // Get global state to get authentication info
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);

    // Check if the client is authenticated or not
    #[cfg(client)]
    global_state.auth.detect_state();

    // TODO -> move into function
    let handle_log_in = move |_event: Event| {
        #[cfg(client)]
        {
            spawn_local_scoped(cx, async move {
                let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
                global_state.auth.state.set(LoginState::Authenticated);
            });
        }
    };

    view! { cx,
        // Main page header
        header {
            div (class = "flex items-center justify-between w-full md:text-center h-20") {
                div(class = "flex-1") {}
                div(class = "text-gray-700 text-2xl font-semibold py-2") {
                    "Pool Elo - Season 1"
                }
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

        main(style = "my-8") {

            (
                match *global_state.auth.state.get() {
                    LoginState::Authenticated => { view! { cx,



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
                    }},
                    _ => { view! { cx, div {} } }})

            // Body header
            div {
                div (class = "container mx-auto px-6 py-3") {
                    nav (class = "sm:flex sm:justify-center sm:items-center mt-4 hidden") {
                        div (class = "flex flex-col sm:flex-row"){
                            a(href = "add-game-form",
                              class = "mt-3 text-gray-600 hover:underline sm:mx-3 sm:mt-0"
                            ) { "Add game result" }
                            a(href = "one-v-one-board",
                              class = "mt-3 text-gray-600 hover:underline sm:mx-3 sm:mt-0"
                            ) { "1v1 Leaderboard" }
                            a(href = "overall-board",
                              class = "mt-3 text-gray-600 hover:underline sm:mx-3 sm:mt-0"
                            ) { "Overall Leaderboard" }
                        }
                    }
                }
            }
            // Actual body
            div(class = "container mx-auto px-6") {
                div(class = "md:flex mt-8 md:-mx-4") {
                    div(class = "rounded-md overflow-hidden bg-cover bg-center") {
                        (children)
                    }
                }
            }
        }
    }
}
