# `rupa-md` 📝

**The Content Engine.** Translates Markdown into a reactive VNode tree, supporting custom styles and metadata extraction.

## 🛠️ Key Features

- **`Engine`**: The primary orchestrator for parsing Markdown.
- **`Config`**: Mapping of Markdown tags to custom Rupa Styles.
- **Metadata Extraction**: Automatically handles `href` for links and `src` for images.
- **VNode Integration**: Produces a tree ready for reconciliation or SSR.

## 🚀 Usage

```rust
use rupa_md::{Engine, Config};
use rupa_vnode::Style;

// 1. Optional: Define custom styles for tags
let mut config = Config::default();
config.tag_styles.insert("h1".to_string(), Style::new().font_size(32.0).bold());

// 2. Parse Markdown content
let markdown = "# Hello Rupa

This is **reactive** content.";
let vnode = Engine::parse(markdown, Some(&config));

// 3. The result is a standard VNode tree
// (ready to be rendered by any Showroom)
```
