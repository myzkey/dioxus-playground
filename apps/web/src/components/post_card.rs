use crate::Route;
use content::PostSummary;
use dioxus::prelude::*;

/// Card component displaying a post summary.
#[component]
pub fn PostCard(summary: PostSummary) -> Element {
    let date_str = summary.date.format("%B %d, %Y").to_string();

    rsx! {
        article { class: "post-card",
            Link {
                to: Route::Post { slug: summary.slug.clone() },
                class: "post-card-link",

                h2 { class: "post-card-title", "{summary.title}" }
                time { class: "post-card-date", datetime: "{summary.date}", "{date_str}" }
                if !summary.description.is_empty() {
                    p { class: "post-card-description", "{summary.description}" }
                }
                if !summary.tags.is_empty() {
                    div { class: "post-card-tags",
                        for tag in summary.tags.iter() {
                            span { class: "tag", "{tag}" }
                        }
                    }
                }
            }
        }
    }
}
