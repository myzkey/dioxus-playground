use crate::components::PostCard;
use content::get_post_summaries;
use dioxus::prelude::*;

/// Home page displaying a list of blog posts.
#[component]
pub fn Home() -> Element {
    let summaries = get_post_summaries();

    rsx! {
        div { class: "home-page",
            h1 { class: "page-title", "Blog Posts" }
            if summaries.is_empty() {
                p { class: "no-posts", "No posts yet. Check back soon!" }
            } else {
                div { class: "post-list",
                    for summary in summaries {
                        PostCard { key: "{summary.slug}", summary }
                    }
                }
            }
        }
    }
}
