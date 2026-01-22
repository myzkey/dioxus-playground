//! Markdown processing library for the blog.
//!
//! Provides markdown to HTML conversion with XSS sanitization.

mod parser;
mod sanitizer;

pub use parser::markdown_to_html;
pub use sanitizer::{add_external_link_targets, sanitize_html};

/// Process markdown content to safe HTML.
///
/// This function:
/// 1. Converts markdown to HTML with GFM extensions
/// 2. Sanitizes the HTML to prevent XSS attacks
/// 3. Adds `target="_blank"` to external links
pub fn process_markdown(markdown: &str) -> String {
    let html = markdown_to_html(markdown);
    let sanitized = sanitize_html(&html);
    add_external_link_targets(&sanitized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_markdown() {
        let md = "# Hello\n\nVisit [Example](https://example.com)!";
        let html = process_markdown(md);

        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("target=\"_blank\""));
    }

    #[test]
    fn test_xss_prevention() {
        let md = "Hello <script>alert('xss')</script> World";
        let html = process_markdown(md);

        assert!(!html.contains("<script>"));
        assert!(html.contains("Hello"));
        assert!(html.contains("World"));
    }
}
