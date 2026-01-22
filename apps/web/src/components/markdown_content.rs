use dioxus::prelude::*;

/// Component to render sanitized HTML content.
///
/// Uses `dangerous_inner_html` to render pre-sanitized HTML from markdown.
/// The content must be sanitized before being passed to this component.
#[component]
pub fn MarkdownContent(html: String) -> Element {
    rsx! {
        div {
            class: "markdown-content",
            dangerous_inner_html: "{html}"
        }
    }
}
