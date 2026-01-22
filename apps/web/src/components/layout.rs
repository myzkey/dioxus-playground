use crate::Route;
use dioxus::prelude::*;

/// Main layout component with header, content, and footer.
#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "layout",
            Header {}
            main { class: "main-content",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}

/// Site header with navigation.
#[component]
fn Header() -> Element {
    rsx! {
        header { class: "header",
            div { class: "header-content",
                Link { to: Route::Home {}, class: "site-title",
                    "Dioxus Blog"
                }
                nav { class: "nav",
                    Link { to: Route::Home {}, class: "nav-link",
                        "Home"
                    }
                    Link { to: Route::About {}, class: "nav-link",
                        "About"
                    }
                }
            }
        }
    }
}

/// Site footer.
#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            p { "Built with Dioxus and Rust" }
        }
    }
}
