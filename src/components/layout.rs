use crate::{
    capsules::{
        forgot_password_form::{ForgotPasswordFormProps, FORGOT_PASSWORD_FORM},
        login_form::{LoginFormProps, LOGIN_FORM},
        register_form::{RegisterFormProps, REGISTER_FORM},
    },
    components::header::Header,
    global_state::AppStateRx,
    state_enums::{ContentState, OpenState},
};
use perseus::prelude::*;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    pub content_state: ContentState,
    pub children: Children<'a, G>,
}

// Using elements from here: https://flowbite.com/docs/components/buttons/

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps {
        content_state,
        children,
    }: LayoutProps<'a, G>,
) -> View<G> {
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
    // Set the theme
    global_state.style.theme.current.set(match content_state {
        ContentState::None => (*global_state.style.theme.default.get()).clone(),
        ContentState::Pool => (*global_state.style.theme.pool.get()).clone(),
        ContentState::Pickleball => (*global_state.style.theme.pickleball.get()).clone(),
        ContentState::TableTennis => (*global_state.style.theme.table_tennis.get()).clone(),
    });
    #[cfg(client)]
    let _ = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .document_element()
        .unwrap()
        .set_attribute("data-theme", &global_state.style.theme.current.get());

    let children = children.call(cx);

    // Check if the client is authenticated or not
    #[cfg(client)]
    global_state.auth.detect_state();

    let content_state_header = content_state.clone();

    view! { cx,
        // Main page header, including login functionality
        Header(content_state = content_state_header)

        // Modals
        section(class = "flex-2") {
            (match *global_state.modals_open.login.get() {
                OpenState::Open => {
                    view! { cx,
                        (LOGIN_FORM.widget(cx, "",
                            LoginFormProps{
                                remember_me: true,
                            }
                        ))
                    }
                }
                OpenState::Closed => {
                    view!{ cx, }
                }
            })
            (match *global_state.modals_open.register.get() {
                OpenState::Open => {
                    view! { cx,
                        (REGISTER_FORM.widget(cx, "",
                            RegisterFormProps{
                                registration_code: true,
                                nickname: true,
                                email: true,
                            }
                        ))
                    }
                }
                OpenState::Closed => {
                    view!{ cx, }
                }
            })
            (match *global_state.modals_open.forgot_password.get() {
                OpenState::Open => {
                    view! { cx,
                        (FORGOT_PASSWORD_FORM.widget(cx, "",
                            ForgotPasswordFormProps{}
                        ))
                    }
                }
                OpenState::Closed => {
                    view!{ cx, }
                }
            })
        }

        main(style = "my-8") {
            (match content_state {
                ContentState::None => view!{ cx, },
                ContentState::Pool => view!{ cx,
                    // Body header
                    div (class = "container mx-auto px-6 py-3") {
                        nav (class = "sm:flex sm:justify-center sm:items-center mt-4 hidden") {
                            div (class = "flex flex-col sm:flex-row"){
                                a(href = "pool/add-game-form",
                                    class = "mt-3 text-gray-600 hover:underline sm:mx-3 sm:mt-0"
                                ) { "Add game result" }
                                a(href = "pool/one-v-one-board",
                                    class = "mt-3 text-gray-600 hover:underline sm:mx-3 sm:mt-0"
                                ) { "1v1 Leaderboard" }
                                a(href = "pool/overall-board",
                                    class = "mt-3 text-gray-600 hover:underline sm:mx-3 sm:mt-0"
                                ) { "Overall Leaderboard" }
                            }
                        }
                    }
                    // Content body
                    div(class = "container mx-auto px-6") {
                        div(class = "md:flex mt-8 md:-mx-4") {
                            div(class = "rounded-md overflow-hidden bg-cover bg-center") {
                                (children)
                            }
                        }
                    }
                },
                ContentState::Pickleball => view!{ cx, },
                ContentState::TableTennis => view!{ cx, },
            })
        }
    }
}
