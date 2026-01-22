//! Dioxus Markdown Blog - Web Application

use dioxus::prelude::*;

mod components;
mod pages;

use components::Layout;
use pages::{About, Home, NotFound, Post};

/// Application routes.
#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/posts/:slug")]
    Post { slug: String },
    #[route("/about")]
    About {},
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("Failed to initialize logger");
    tracing::info!("Starting Dioxus Markdown Blog");
    dioxus::launch(App);
}

/// Root application component.
#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
