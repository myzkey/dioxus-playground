use crate::Route;
use dioxus::prelude::*;

/// 404 Not Found page.
#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let path = format!("/{}", route.join("/"));

    rsx! {
        div { class: "not-found-page",
            h1 { "404 - Page Not Found" }
            p { "The page \"{path}\" does not exist." }
            Link { to: Route::Home {}, class: "home-link",
                "← Go to Home"
            }
        }
    }
}
