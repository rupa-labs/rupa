# Specialized Artisan Showrooms: The Showrooms 🏪

Specialized Artisan Showrooms are the **Tier 3** entry points of the Rupa Framework. They represent the "Showroom" where Atomic Materials and Composite Assemblies are pre-assembled for specific business use cases.

---

## 1. Why Specialized Artisan Showrooms?

To balance flexibility and ease of use, Rupa provides dedicated crates that bundle only what is necessary for a specific target:
- **Lightweight**: Users don't pull in heavy GPU dependencies for a CLI tool.
- **Opinionated**: Pre-configured preludes and defaults for rapid development.
- **Standardized**: Every artisan showroom follows the same Rupa reactive philosophy.

---

## 2. Available Artisan Showrooms

| Artisan Showroom | Purpose | Pre-assembled Systems |
| :--- | :--- | :--- |
| **`rupa-desktop`** | Native GUI | Core + UI + Engine(GPU) + Assets + Motion |
| **`rupa-web`** | Web Frontend | Core + UI + Web Core + Router + Auth |
| **`rupa-server`** | SSR / Backend | Core + Server Core + Store + Auth + i18n |
| **`rupa-tui`** | TUI / Tooling | Core + Engine(TUI) + Signals + Telemetry |
| **`rupa-mobile`** | Native Mobile | Core + UI + Engine(GPU) + Mobile Core + Assets |
| **`rupa-fullstack`**| Universal | UI + Server Core + Web Core + Router + Store + Auth |
| **`rupa-headless`** | Logic Only | Core + Signals + VNode + Store + Net |
| **`rupa-hybrid`** | JS/TS Bridge | Core + Web Core + VNode + Serialization |
