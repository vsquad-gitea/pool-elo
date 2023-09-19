use sycamore::prelude::*;

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    /// The title of the page, which will be displayed in the header.
    pub title: &'a str,
    /// The content to put inside the layout.
    pub children: Children<'a, G>,
}

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps { title, children }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);

    // example params
    // p { (title.to_string()) }

    view! { cx,
        // These elements are styled with bright colors for demonstration purposes
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
                    div(class = "h-64 rounded-md overflow-hidden bg-cover bg-center") {
                        (children)
                    }
                }
            }
        }
        footer(class = "bg-gray-200") {
            p(class = "container mx-auto px-6 py-3 flex justify-between items-center"){
                "Hey there, I'm a footer!"
            }
        }
    }
}
