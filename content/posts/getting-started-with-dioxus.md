---
title: Getting Started with Dioxus
date: 2024-01-20
description: A beginner's guide to building web applications with Dioxus and Rust.
tags:
  - rust
  - dioxus
  - tutorial
---

# Getting Started with Dioxus

Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust.

## Installation

First, install the Dioxus CLI:

```bash
cargo install dioxus-cli
```

## Creating a New Project

Create a new Dioxus project:

```bash
dx new my-app
cd my-app
```

## Your First Component

Here's a simple counter component:

```rust
use dioxus::prelude::*;

fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            h1 { "Counter: {count}" }
            button { onclick: move |_| count += 1, "Increment" }
            button { onclick: move |_| count -= 1, "Decrement" }
        }
    }
}
```

## Running Your App

Start the development server:

```bash
dx serve
```

Open http://localhost:8080 in your browser!

## Next Steps

- Check out the [official documentation](https://dioxuslabs.com/docs/)
- Join the [Discord community](https://discord.gg/XgGxMSkvUM)
- Explore the [examples repository](https://github.com/DioxusLabs/dioxus/tree/main/examples)
