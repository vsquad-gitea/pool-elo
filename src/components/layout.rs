use crate::{
    capsules::login_form::{LoginFormProps, LOGIN_FORM},
    components::header::{Header, HeaderProps},
    global_state::AppStateRx,
    state_enums::{GameState, LoginState},
};
use perseus::prelude::*;
use sycamore::prelude::*;
use web_sys::Event;

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    pub game: GameState,
    pub title: &'a str,
    pub children: Children<'a, G>,
}

// Using elements from here: https://flowbite.com/docs/components/buttons/

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps {
        game,
        title,
        children,
    }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);

    // Get global state to get authentication info
    #[cfg(client)]
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);

    // Check if the client is authenticated or not
    #[cfg(client)]
    global_state.auth.detect_state();

    view! { cx,
        // Main page header, including login functionality
        Header(game = game, title = title)

        main(style = "my-8") {
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
