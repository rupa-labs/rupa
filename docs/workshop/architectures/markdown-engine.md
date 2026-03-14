# Markdown Engine Architecture 📖

The **Markdown Engine** is a Tier 2 system that transforms content-driven sources (MD/MDX) into universal `VNode` trees.

---

## 1. Core Workflow

1. **Parsing**: Consumes raw Markdown strings using the `pulldown-cmark` adapter.
2. **Mapping**: Translates semantic Markdown tags (`h1`, `p`, `li`) into Rupa **Agnostic UI** components.
3. **Reactive Integration**: Supports embedding reactive Artisan components directly into content (MDX).

---

## 2. Technical Characteristics

- **Agnostic Output**: Produces a standard VNode tree that can be rendered by any **Physical Adapter**.
- **Themable**: Mapping rules can be configured to use specific design tokens or custom components.
- **SEO Ready**: Facilitates Static Site Generation (SSG) by providing a serializable UI structure.
