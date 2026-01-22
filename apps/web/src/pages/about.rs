use dioxus::prelude::*;

/// About page with site information.
#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "space-y-8",
            h1 { class: "text-3xl font-bold text-slate-900 dark:text-white mb-8",
                "About"
            }
            div { class: "prose prose-slate dark:prose-invert max-w-none",
                p { class: "text-lg text-slate-600 dark:text-slate-300",
                    "Welcome to the Dioxus Markdown Blog! This is a demonstration of building "
                    "a static blog using Rust and the Dioxus web framework."
                }

                h2 { class: "text-xl font-semibold text-slate-900 dark:text-white mt-8 mb-4",
                    "Technology Stack"
                }
                ul { class: "space-y-2 text-slate-600 dark:text-slate-300 list-disc list-inside",
                    li {
                        span { class: "font-semibold", "Dioxus 0.7" }
                        " - A Rust framework for building user interfaces"
                    }
                    li {
                        span { class: "font-semibold", "Tailwind CSS" }
                        " - Utility-first CSS framework"
                    }
                    li {
                        span { class: "font-semibold", "pulldown-cmark" }
                        " - Markdown parsing with GFM support"
                    }
                    li {
                        span { class: "font-semibold", "ammonia" }
                        " - HTML sanitization for XSS protection"
                    }
                    li {
                        span { class: "font-semibold", "WebAssembly" }
                        " - Runs entirely in the browser"
                    }
                }

                h2 { class: "text-xl font-semibold text-slate-900 dark:text-white mt-8 mb-4",
                    "Features"
                }
                ul { class: "space-y-2 text-slate-600 dark:text-slate-300 list-disc list-inside",
                    li { "Markdown rendering with syntax highlighting support" }
                    li { "YAML front matter for post metadata" }
                    li { "Responsive design with dark mode support" }
                    li { "XSS-safe HTML rendering" }
                    li { "External links open in new tabs" }
                }
            }
        }
    }
}
