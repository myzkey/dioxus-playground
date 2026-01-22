---
title: Error Handling in Rust
date: 2024-01-25
description: Learn how to handle errors effectively in Rust using Result, Option, and the ? operator.
tags:
  - rust
  - error-handling
  - best-practices
---

# Error Handling in Rust

Rust's approach to error handling is one of its most powerful features. Let's explore the different patterns.

## The Result Type

The `Result` type is used for operations that can fail:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Example

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

## The ? Operator

The `?` operator makes error propagation concise:

```rust
fn read_config() -> Result<Config, Error> {
    let content = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

## Custom Error Types

Use `thiserror` for easy custom errors:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),
}
```

## Best Practices

1. **Be specific** - Use descriptive error types
2. **Don't panic** - Use `Result` instead of `unwrap()` in library code
3. **Provide context** - Add context with `anyhow` or custom messages
4. **Log errors** - Use `tracing` for structured logging

## Conclusion

Rust's error handling may feel verbose at first, but it leads to more robust and maintainable code.
