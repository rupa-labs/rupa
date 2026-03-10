# Content-Driven Architecture (SSG & Markdown) 📄

Rupa Framework supports building content-heavy websites using a specialized Static Site Generation (SSG) pipeline. Inspired by the efficiency of Astro, Rupa allows you to treat Markdown files as first-class UI materials.

---

## 🏗️ The SSG Pipeline

The transformation from raw content to a static site follows this flow:
1.  **Source Discovery**: The `rupa-cli` scans the `src/pages` directory for `.rs`, `.md`, or `.mdx` files.
2.  **VNode Conversion**: 
    - `.rs` components are executed to produce VNodes.
    - `.md/.mdx` files are parsed by `rupa-md` into a corresponding VNode tree.
3.  **Agnostic Rendering**: The `HtmlRenderer` in `rupa-server-core` transforms the VNode tree into optimized, minified HTML.
4.  **Static Export**: Files are written to the `dist/` directory, ready for deployment on any static host.

---

## 📝 Markdown & MDX Support

Rupa treats Markdown as a collection of semantic components.

### 1. Frontmatter Integration
Metadata defined in the YAML frontmatter is automatically injected into the component's reactive `Signal` context.

### 2. Component Mapping
You can map standard Markdown tags to custom Rupa components:
```rust
// Map # Header to our aesthetic Brand component
md_config.map("h1", |text| Brand::new(text).font_size(32.0));
```

### 3. MDX: Interactive Markdown
MDX allows you to embed reactive Rupa components directly inside your Markdown files:
```markdown
# My Artisan Post

Check out this reactive counter:
<Counter initial={10} />
```

---

## 🚀 Commands

### Development
```bash
rupa dev # Starts a local server with HMR
```

### Production Build
```bash
rupa build # Generates static HTML in /dist
```

---

## 🛠️ Implementation Strategy

1.  **`rupa-md`**: A new Tier 1 crate using `pulldown-cmark` to parse Markdown into `VNode::Element`.
2.  **`rupa-server-core`**: Implement `VNode` -> `HTML` serialization.
3.  **`rupa-cli`**: Implement the `build` command and file-system routing logic.
