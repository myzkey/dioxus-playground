use crate::components::PostCard;
use content::get_post_summaries;
use dioxus::prelude::*;

/// Home page displaying a list of blog posts.
#[component]
pub fn Home() -> Element {
    let summaries = get_post_summaries();

    rsx! {
        div { class: "space-y-12",
            // Hero section
            header { class: "text-center py-8",
                h1 { class: "text-4xl sm:text-5xl font-extrabold text-slate-900 dark:text-white mb-4 tracking-tight",
                    "Welcome to the Blog"
                }
                p { class: "text-lg text-slate-600 dark:text-slate-400 max-w-2xl mx-auto",
                    "Exploring Rust, WebAssembly, and modern web development with Dioxus."
                }
            }

            // Posts section
            section {
                div { class: "flex items-center justify-between mb-8",
                    h2 { class: "text-2xl font-bold text-slate-900 dark:text-white",
                        "Latest Posts"
                    }
                    span { class: "text-sm text-slate-500 dark:text-slate-400",
                        "{summaries.len()} articles"
                    }
                }

                if summaries.is_empty() {
                    div { class: "text-center py-16 bg-white dark:bg-slate-900 rounded-2xl border border-slate-200 dark:border-slate-800",
                        div { class: "w-16 h-16 mx-auto mb-4 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center",
                            svg {
                                class: "w-8 h-8 text-slate-400",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"
                                }
                            }
                        }
                        p { class: "text-slate-500 dark:text-slate-400",
                            "No posts yet. Check back soon!"
                        }
                    }
                } else {
                    div { class: "grid gap-6",
                        for summary in summaries {
                            PostCard { key: "{summary.slug}", summary }
                        }
                    }
                }
            }
        }
    }
}
