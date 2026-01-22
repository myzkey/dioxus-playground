use dioxus::prelude::*;

/// About page with site information.
#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "about-page",
            h1 { class: "page-title", "About" }
            div { class: "about-content",
                p {
                    "Welcome to the Dioxus Markdown Blog! This is a demonstration of building "
                    "a static blog using Rust and the Dioxus web framework."
                }
                h2 { "Technology Stack" }
                ul {
                    li { strong { "Dioxus 0.6" } " - A Rust framework for building user interfaces" }
                    li { strong { "pulldown-cmark" } " - Markdown parsing with GFM support" }
                    li { strong { "ammonia" } " - HTML sanitization for XSS protection" }
                    li { strong { "WebAssembly" } " - Runs entirely in the browser" }
                }
                h2 { "Features" }
                ul {
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
