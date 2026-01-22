use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// YAML front matter for a blog post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostFrontMatter {
    pub title: String,
    pub date: NaiveDate,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub draft: bool,
}

/// A full blog post with front matter and content.
#[derive(Debug, Clone)]
pub struct Post {
    pub slug: String,
    pub front_matter: PostFrontMatter,
    pub content: String,
    pub html_content: String,
}

impl Post {
    /// Create a new Post.
    pub fn new(
        slug: String,
        front_matter: PostFrontMatter,
        content: String,
        html_content: String,
    ) -> Self {
        Self {
            slug,
            front_matter,
            content,
            html_content,
        }
    }

    /// Get the post title.
    pub fn title(&self) -> &str {
        &self.front_matter.title
    }

    /// Get the post date.
    pub fn date(&self) -> NaiveDate {
        self.front_matter.date
    }

    /// Get the post description.
    pub fn description(&self) -> &str {
        &self.front_matter.description
    }

    /// Get the post tags.
    pub fn tags(&self) -> &[String] {
        &self.front_matter.tags
    }

    /// Check if the post is a draft.
    pub fn is_draft(&self) -> bool {
        self.front_matter.draft
    }
}

/// Summary of a blog post for listing pages.
#[derive(Debug, Clone, PartialEq)]
pub struct PostSummary {
    pub slug: String,
    pub title: String,
    pub date: NaiveDate,
    pub description: String,
    pub tags: Vec<String>,
}

impl From<&Post> for PostSummary {
    fn from(post: &Post) -> Self {
        Self {
            slug: post.slug.clone(),
            title: post.front_matter.title.clone(),
            date: post.front_matter.date,
            description: post.front_matter.description.clone(),
            tags: post.front_matter.tags.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_summary_from_post() {
        let front_matter = PostFrontMatter {
            title: "Test Post".to_string(),
            date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            description: "A test post".to_string(),
            tags: vec!["rust".to_string(), "test".to_string()],
            draft: false,
        };

        let post = Post::new(
            "test-post".to_string(),
            front_matter,
            "Content".to_string(),
            "<p>Content</p>".to_string(),
        );

        let summary = PostSummary::from(&post);
        assert_eq!(summary.slug, "test-post");
        assert_eq!(summary.title, "Test Post");
        assert_eq!(summary.tags.len(), 2);
    }
}
