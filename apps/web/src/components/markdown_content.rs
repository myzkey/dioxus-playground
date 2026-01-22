use dioxus::prelude::*;

/// Component to render sanitized HTML content.
///
/// Uses `dangerous_inner_html` to render pre-sanitized HTML from markdown.
/// The content must be sanitized before being passed to this component.
#[component]
pub fn MarkdownContent(html: String) -> Element {
    rsx! {
        div {
            class: "prose prose-slate dark:prose-invert max-w-none
                prose-headings:font-semibold
                prose-h1:text-3xl prose-h2:text-2xl prose-h3:text-xl
                prose-a:text-blue-500 prose-a:no-underline hover:prose-a:underline
                prose-code:bg-slate-100 prose-code:dark:bg-slate-800 prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded prose-code:text-sm prose-code:before:content-none prose-code:after:content-none
                prose-pre:bg-slate-100 prose-pre:dark:bg-slate-800 prose-pre:rounded-lg
                prose-blockquote:border-blue-500 prose-blockquote:italic
                prose-img:rounded-lg",
            dangerous_inner_html: "{html}"
        }
    }
}
