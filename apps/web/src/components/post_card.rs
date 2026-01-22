use crate::Route;
use content::PostSummary;
use dioxus::prelude::*;

/// Card component displaying a post summary.
#[component]
pub fn PostCard(summary: PostSummary) -> Element {
    let date_str = summary.date.format("%B %d, %Y").to_string();

    rsx! {
        article { class: "group relative bg-white dark:bg-slate-900 rounded-2xl border border-slate-200 dark:border-slate-800 overflow-hidden hover:border-slate-300 dark:hover:border-slate-700 hover:shadow-xl hover:shadow-slate-200/50 dark:hover:shadow-slate-900/50 transition-all duration-300",
            Link {
                to: Route::Post { slug: summary.slug.clone() },
                class: "block p-6",

                // Date badge
                div { class: "flex items-center gap-2 mb-3",
                    div { class: "w-1.5 h-1.5 rounded-full bg-blue-500" }
                    time {
                        class: "text-xs font-medium text-slate-500 dark:text-slate-400 uppercase tracking-wide",
                        datetime: "{summary.date}",
                        "{date_str}"
                    }
                }

                // Title
                h2 { class: "text-xl font-bold text-slate-900 dark:text-white mb-2 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors",
                    "{summary.title}"
                }

                // Description
                if !summary.description.is_empty() {
                    p { class: "text-slate-600 dark:text-slate-400 leading-relaxed mb-4",
                        "{summary.description}"
                    }
                }

                // Tags
                if !summary.tags.is_empty() {
                    div { class: "flex flex-wrap gap-2",
                        for tag in summary.tags.iter() {
                            span { class: "px-2.5 py-1 text-xs font-medium bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 rounded-full",
                                "{tag}"
                            }
                        }
                    }
                }

                // Arrow indicator
                div { class: "absolute bottom-6 right-6 w-8 h-8 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center opacity-0 group-hover:opacity-100 translate-x-2 group-hover:translate-x-0 transition-all",
                    svg {
                        class: "w-4 h-4 text-slate-600 dark:text-slate-400",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M9 5l7 7-7 7"
                        }
                    }
                }
            }
        }
    }
}
