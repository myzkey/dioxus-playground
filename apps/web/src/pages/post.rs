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
                article { class: "max-w-3xl mx-auto",
                    // Back link
                    Link {
                        to: Route::Home {},
                        class: "inline-flex items-center gap-2 text-sm font-medium text-slate-500 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white mb-8 transition-colors",
                        svg {
                            class: "w-4 h-4",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M15 19l-7-7 7-7"
                            }
                        }
                        "Back to posts"
                    }

                    // Header
                    header { class: "mb-10",
                        // Date
                        div { class: "flex items-center gap-2 mb-4",
                            div { class: "w-1.5 h-1.5 rounded-full bg-blue-500" }
                            time {
                                class: "text-sm font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wide",
                                datetime: "{post.date()}",
                                "{date_str}"
                            }
                        }

                        // Title
                        h1 { class: "text-3xl sm:text-4xl font-extrabold text-slate-900 dark:text-white mb-6 leading-tight tracking-tight",
                            "{post.title()}"
                        }

                        // Tags
                        if !tags.is_empty() {
                            div { class: "flex flex-wrap gap-2",
                                for tag in tags.iter() {
                                    span { class: "px-3 py-1 text-sm font-medium bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 rounded-full",
                                        "{tag}"
                                    }
                                }
                            }
                        }
                    }

                    // Divider
                    div { class: "w-full h-px bg-gradient-to-r from-transparent via-slate-200 dark:via-slate-700 to-transparent mb-10" }

                    // Content
                    div { class: "prose prose-slate dark:prose-invert prose-lg max-w-none",
                        MarkdownContent { html: post.html_content.clone() }
                    }

                    // Footer
                    footer { class: "mt-16 pt-8 border-t border-slate-200 dark:border-slate-800",
                        Link {
                            to: Route::Home {},
                            class: "inline-flex items-center gap-2 px-6 py-3 bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 font-medium rounded-xl hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors",
                            svg {
                                class: "w-4 h-4",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M15 19l-7-7 7-7"
                                }
                            }
                            "Back to all posts"
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "text-center py-20",
                    div { class: "w-20 h-20 mx-auto mb-6 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center",
                        svg {
                            class: "w-10 h-10 text-slate-400",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z"
                            }
                        }
                    }
                    h1 { class: "text-2xl font-bold text-slate-900 dark:text-white mb-3",
                        "Post Not Found"
                    }
                    p { class: "text-slate-500 dark:text-slate-400 mb-8",
                        "The post \"{slug}\" could not be found."
                    }
                    Link {
                        to: Route::Home {},
                        class: "inline-flex items-center gap-2 px-6 py-3 bg-blue-500 hover:bg-blue-600 text-white font-medium rounded-xl transition-colors",
                        "Go to Home"
                    }
                }
            }
        }
    }
}
