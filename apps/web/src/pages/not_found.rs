use crate::Route;
use dioxus::prelude::*;

/// 404 Not Found page.
#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let path = format!("/{}", route.join("/"));

    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-[60vh] text-center px-4",
            // 404 badge
            div { class: "mb-8",
                span { class: "inline-block px-4 py-2 bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400 text-sm font-semibold rounded-full",
                    "Error 404"
                }
            }

            // Large 404
            h1 { class: "text-8xl sm:text-9xl font-black text-slate-200 dark:text-slate-800 mb-4",
                "404"
            }

            // Message
            h2 { class: "text-2xl sm:text-3xl font-bold text-slate-900 dark:text-white mb-4",
                "Page not found"
            }
            p { class: "text-slate-500 dark:text-slate-400 mb-8 max-w-md",
                "Sorry, we couldn't find the page \""
                span { class: "font-mono text-sm bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded", "{path}" }
                "\". It might have been moved or deleted."
            }

            // Action buttons
            div { class: "flex flex-col sm:flex-row gap-3",
                Link {
                    to: Route::Home {},
                    class: "inline-flex items-center justify-center gap-2 px-6 py-3 bg-blue-500 hover:bg-blue-600 text-white font-medium rounded-xl transition-colors",
                    svg {
                        class: "w-5 h-5",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
                        }
                    }
                    "Go Home"
                }
            }
        }
    }
}
