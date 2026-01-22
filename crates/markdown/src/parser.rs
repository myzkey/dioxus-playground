use pulldown_cmark::{Options, Parser, html};

/// Convert markdown text to HTML string.
///
/// Enables GFM extensions: tables, strikethrough, tasklists.
pub fn markdown_to_html(markdown: &str) -> String {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES;

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown() {
        let md = "# Hello\n\nThis is a paragraph.";
        let html = markdown_to_html(md);
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<p>This is a paragraph.</p>"));
    }

    #[test]
    fn test_tables() {
        let md = "| Header |\n|--------|\n| Cell |";
        let html = markdown_to_html(md);
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>Header</th>"));
        assert!(html.contains("<td>Cell</td>"));
    }

    #[test]
    fn test_strikethrough() {
        let md = "~~deleted~~";
        let html = markdown_to_html(md);
        assert!(html.contains("<del>deleted</del>"));
    }

    #[test]
    fn test_code_blocks() {
        let md = "```rust\nfn main() {}\n```";
        let html = markdown_to_html(md);
        assert!(html.contains("<code"));
        assert!(html.contains("fn main()"));
    }
}
