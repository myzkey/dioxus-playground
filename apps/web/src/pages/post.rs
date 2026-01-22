use crate::components::MarkdownContent;
use crate::Route;
use content::find_post_by_slug;
use dioxus::prelude::*;

/// Individual post page displaying full article content.
#[component]
pub fn Post(slug: String) -> Element {
    let post = find_post_by_slug(&slug);

    match post {
        Some(post) => {
            let date_str = post.date().format("%B %d, %Y").to_string();
            let tags = post.tags().to_vec();

            rsx! {
                article { class: "post-page",
                    header { class: "post-header",
                        h1 { class: "post-title", "{post.title()}" }
                        time {
                            class: "post-date",
                            datetime: "{post.date()}",
                            "{date_str}"
                        }
                        if !tags.is_empty() {
                            div { class: "post-tags",
                                for tag in tags.iter() {
                                    span { class: "tag", "{tag}" }
                                }
                            }
                        }
                    }
                    MarkdownContent { html: post.html_content.clone() }
                    footer { class: "post-footer",
                        Link { to: Route::Home {}, class: "back-link",
                            "← Back to posts"
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "post-not-found",
                    h1 { "Post Not Found" }
                    p { "The post \"{slug}\" could not be found." }
                    Link { to: Route::Home {}, class: "back-link",
                        "← Back to posts"
                    }
                }
            }
        }
    }
}
