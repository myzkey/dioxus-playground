use ammonia::Builder;
use std::collections::HashSet;

/// Sanitize HTML to prevent XSS attacks.
///
/// Allows common HTML elements used in markdown output while
/// removing potentially dangerous elements like `<script>`.
pub fn sanitize_html(html: &str) -> String {
    let mut builder = Builder::default();

    // Allow class attributes for syntax highlighting
    let mut allowed_classes = HashSet::new();
    allowed_classes.insert("language-rust");
    allowed_classes.insert("language-javascript");
    allowed_classes.insert("language-typescript");
    allowed_classes.insert("language-python");
    allowed_classes.insert("language-html");
    allowed_classes.insert("language-css");
    allowed_classes.insert("language-json");
    allowed_classes.insert("language-bash");
    allowed_classes.insert("language-shell");
    allowed_classes.insert("language-yaml");
    allowed_classes.insert("language-toml");
    allowed_classes.insert("language-markdown");
    allowed_classes.insert("language-sql");

    builder.add_allowed_classes("code", allowed_classes);

    // Allow id attributes for heading anchors
    builder.add_generic_attributes(["id"]);

    builder.clean(html).to_string()
}

/// Add `target="_blank"` and `rel="noopener noreferrer"` to external links.
///
/// External links are identified as those starting with `http://` or `https://`.
pub fn add_external_link_targets(html: &str) -> String {
    // Simple regex-free approach: find <a href="http and add attributes
    let mut result = String::with_capacity(html.len());
    let mut remaining = html;

    while let Some(start) = remaining.find("<a ") {
        result.push_str(&remaining[..start]);
        remaining = &remaining[start..];

        if let Some(end) = remaining.find('>') {
            let tag = &remaining[..=end];

            // Check if it's an external link
            let is_external = tag.contains("href=\"http://") || tag.contains("href=\"https://");

            if is_external && !tag.contains("target=") {
                // Insert target and rel before the closing >
                let modified = format!(
                    "{} target=\"_blank\" rel=\"noopener noreferrer\">",
                    &tag[..end]
                );
                result.push_str(&modified);
            } else {
                result.push_str(tag);
            }

            remaining = &remaining[end + 1..];
        } else {
            result.push_str(remaining);
            break;
        }
    }

    result.push_str(remaining);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_removes_script() {
        let html = "<p>Hello</p><script>alert('xss')</script>";
        let sanitized = sanitize_html(html);
        assert!(sanitized.contains("<p>Hello</p>"));
        assert!(!sanitized.contains("<script>"));
        assert!(!sanitized.contains("alert"));
    }

    #[test]
    fn test_sanitize_allows_common_tags() {
        let html = "<h1>Title</h1><p>Text with <strong>bold</strong> and <em>italic</em>.</p>";
        let sanitized = sanitize_html(html);
        assert!(sanitized.contains("<h1>"));
        assert!(sanitized.contains("<strong>"));
        assert!(sanitized.contains("<em>"));
    }

    #[test]
    fn test_sanitize_allows_links() {
        let html = r#"<a href="https://example.com">Link</a>"#;
        let sanitized = sanitize_html(html);
        assert!(sanitized.contains("<a"));
        assert!(sanitized.contains("href="));
    }

    #[test]
    fn test_sanitize_allows_code_blocks() {
        let html = r#"<pre><code class="language-rust">fn main() {}</code></pre>"#;
        let sanitized = sanitize_html(html);
        assert!(sanitized.contains("<pre>"));
        assert!(sanitized.contains("<code"));
        assert!(sanitized.contains("language-rust"));
    }

    #[test]
    fn test_external_link_targets() {
        let html = r#"<a href="https://example.com">External</a>"#;
        let result = add_external_link_targets(html);
        assert!(result.contains("target=\"_blank\""));
        assert!(result.contains("rel=\"noopener noreferrer\""));
    }

    #[test]
    fn test_internal_link_unchanged() {
        let html = r#"<a href="/about">Internal</a>"#;
        let result = add_external_link_targets(html);
        assert!(!result.contains("target="));
    }

    #[test]
    fn test_existing_target_unchanged() {
        let html = r#"<a href="https://example.com" target="_self">Link</a>"#;
        let result = add_external_link_targets(html);
        assert!(result.contains("target=\"_self\""));
        assert!(!result.contains("target=\"_blank\""));
    }
}
