---
title: Hello World - Welcome to the Dioxus Blog
date: 2024-01-15
description: An introduction to the Dioxus Markdown Blog, showcasing various markdown features and the technology stack behind it.
tags:
  - rust
  - dioxus
  - webassembly
---

# Welcome to the Dioxus Blog

This is the first post on our new blog built with **Dioxus** and **Rust**. This blog demonstrates how to build a modern, fast, and secure web application using Rust's ecosystem.

## Why Dioxus?

Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust. Here are some of its key features:

- **Familiar React-like syntax** with RSX macros
- **Cross-platform support** - web, desktop, mobile, and TUI
- **Type-safe routing** with compile-time checks
- **Excellent performance** thanks to Rust and WebAssembly

## Markdown Features

This blog supports various markdown features through [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark):

### Code Blocks

Here's an example of Rust code with syntax highlighting:

```rust
fn main() {
    println!("Hello, Dioxus!");
}
```

And some JavaScript:

```javascript
const greeting = "Hello, World!";
console.log(greeting);
```

### Tables

| Feature | Status |
|---------|--------|
| Markdown parsing | Supported |
| Code blocks | Supported |
| Tables | Supported |
| ~~Strikethrough~~ | Supported |

### Lists

#### Unordered List

- First item
- Second item
  - Nested item
  - Another nested item
- Third item

#### Ordered List

1. First step
2. Second step
3. Third step

### Blockquotes

> "The best way to predict the future is to invent it."
> — Alan Kay

### Links

Check out these resources:

- [Dioxus Documentation](https://dioxuslabs.com/docs/)
- [Rust Programming Language](https://www.rust-lang.org/)
- [WebAssembly](https://webassembly.org/)

## Security

This blog uses [ammonia](https://github.com/rust-ammonia/ammonia) to sanitize all HTML output, preventing XSS attacks. Even if someone tries to inject malicious scripts:

```html
<script>alert('This will not execute!')</script>
```

The script tags are safely removed from the output.

## What's Next?

In future posts, we'll explore:

1. Building components with Dioxus
2. State management patterns
3. Server-side rendering
4. Deploying to production

Stay tuned for more content!

---

*Thanks for reading! If you have questions or feedback, feel free to reach out.*
