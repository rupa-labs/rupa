# Hybrid Adapter 🤝

The **Hybrid Adapter** provides the infrastructure for blending different runtime environments and technologies within a single Rupa application.

---

## 🏗️ Technical Architecture
- **WebView Bridge**: Uses Tao/Wry to embed web contexts inside native windows.
- **Context Bus**: A high-performance, asynchronous bridge for sharing reactive state between Rust and JavaScript.
- **Platform Interop**: Standardized traits for calling native APIs from web content and vice versa.

---

## 🎨 Artisan Focus
Designed for **Incremental Modernization** and **Specialized Integration**. Perfect for embedding complex web widgets (like rich text editors or mapping tools) into native apps, or when you need to leverage existing web ecosystems alongside Rupa's native power.

[Explore Hybrid Technical Spec](../../workshop/architectures/hybrid-adapter.md)
