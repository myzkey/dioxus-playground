use crate::post::{Post, PostFrontMatter, PostSummary};
use thiserror::Error;
use yaml_front_matter::YamlFrontMatter;

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("Failed to parse front matter: {0}")]
    FrontMatterParse(String),
    #[error("Post not found: {0}")]
    PostNotFound(String),
}

/// Parse a markdown file with YAML front matter into a Post.
pub fn parse_post(slug: &str, content: &str) -> Result<Post, ContentError> {
    let document = YamlFrontMatter::parse::<PostFrontMatter>(content)
        .map_err(|e| ContentError::FrontMatterParse(e.to_string()))?;

    let html_content = markdown::process_markdown(&document.content);

    Ok(Post::new(
        slug.to_string(),
        document.metadata,
        document.content,
        html_content,
    ))
}

/// Load all posts from included content.
///
/// This uses build-time loading via include_str! for simplicity
/// and WASM compatibility.
pub fn load_all_posts() -> Vec<Post> {
    let mut posts = Vec::new();

    // Include posts at build time
    let hello_world = include_str!("../../../content/posts/hello-world.md");
    if let Ok(post) = parse_post("hello-world", hello_world)
        && !post.is_draft()
    {
        posts.push(post);
    }

    // Sort by date descending
    posts.sort_by_key(|p| std::cmp::Reverse(p.date()));

    posts
}

/// Get summaries of all posts.
pub fn get_post_summaries() -> Vec<PostSummary> {
    load_all_posts().iter().map(PostSummary::from).collect()
}

/// Find a post by its slug.
pub fn find_post_by_slug(slug: &str) -> Option<Post> {
    load_all_posts().into_iter().find(|p| p.slug == slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_post() {
        let content = r#"---
title: Test Post
date: 2024-01-15
description: A test post
tags:
  - rust
  - test
---

# Hello World

This is the content."#;

        let post = parse_post("test-post", content).unwrap();
        assert_eq!(post.title(), "Test Post");
        assert_eq!(post.slug, "test-post");
        assert!(post.html_content.contains("<h1>Hello World</h1>"));
    }

    #[test]
    fn test_parse_post_minimal() {
        let content = r#"---
title: Minimal
date: 2024-01-01
---

Content only."#;

        let post = parse_post("minimal", content).unwrap();
        assert_eq!(post.title(), "Minimal");
        assert!(post.description().is_empty());
        assert!(post.tags().is_empty());
    }
}
