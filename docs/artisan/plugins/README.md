# Artisan Plugins (The Official Kit) 🧩

Plugins are the primary way to extend your Rupa application with specialized capabilities. While the framework provides the foundational material, **Official Artisan Plugins** are pre-built products that solve complex business needs.

---

## 🛡️ Guild Mastery (via `rupa-guild`)
**The Official Identity & Access System.**
A complete, reactive solution for managing Users, Teams, and RBAC (Role-Based Access Control) within any Rupa application.

- **Identity**: Standard models for Users and Tenants.
- **Enforcement**: Built-in Gate and Policy engines.
- **Adapters**: Ready-to-use adapters for common auth protocols.

[**Explore the Guild Guide →**](./guild-mastery.md)

---

## 📖 Using Plugins
To integrate a plugin, simply add it during your app's initialization:

```rust
let app = App::new("My Artisan App")
    .plugin(GuildPlugin::new()) // Add official kits here
    .root(MyComponent);
```

---

*New official kits are constantly being crafted in the workshop. Stay tuned for more.*
