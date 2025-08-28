# Welcome to the Note

This value comes from Rust: **{{ stringvalue }}**

## Features Demo

~~This text is strikethrough~~
<u>This text is underlined</u>

Here is a list of items:

{% for item in vec_strings %}
- {{ item }}
{% endfor %}

## Table Example

| Language | Status | Notes |
|----------|--------|-------|
| Rust     | ✅     | Amazing performance |
| Python   | ✅     | Great for scripting |
| JavaScript | ⚠️   | Ecosystem is huge |

## Task List

- [x] Set up Axum server
- [x] Configure Askama templates  
- [x] Add markdown support
- [ ] Add more features
- [ ] Deploy to production

## Code Block

```rust
fn main() {
    println!("Hello from Rust!");
}
```

## LaTeX Example

You can include LaTeX equations like this:

Inline math $`1 + 2`$

```math
x^2
```

> **Note**: This markdown now supports tables, strikethrough, task lists, and more!
