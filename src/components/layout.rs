use sycamore::prelude::*;

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    pub _title: &'a str,
    pub children: Children<'a, G>,
}

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps {
        _title: _,
        children,
    }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);

    view! { cx,
        header {
            div (class = "flex items-center justify-between") {
                div (class = "w-full text-gray-700 md:text-center text-2xl font-semibold") {
                    "Pool Elo - Season 1"
                }
            }

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

        main(style = "my-8") {
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
