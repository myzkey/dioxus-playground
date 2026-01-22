//! Content management library for the blog.
//!
//! Provides post data structures and loading functionality.

mod loader;
mod post;

pub use loader::{find_post_by_slug, get_post_summaries, load_all_posts, parse_post, ContentError};
pub use post::{Post, PostFrontMatter, PostSummary};
