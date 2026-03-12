# `rupa-mobile` 📱

**The Native Mobile Showroom.** This crate provides the **Adapters & Infrastructure** (Tier 3) to run Rupa applications natively on iOS and Android devices.

---

## 🏛️ Architectural Role
- **Tier**: Tier 3 (Showrooms)
- **Identity**: The Finished Showroom (Adapters & Infrastructure)
- **Responsibility**: Bridges the core engine with mobile OS lifecycles (Active/Background) and hardware features (Haptics, Sensors).

## 🛠️ Infrastructure Backends
- **Windowing**: `winit` (Android/iOS targets)
- **NDK**: `ndk` and `ndk-context` (Android)
- **Graphics**: `wgpu` (Metal/Vulkan)

## 🚀 Usage

```rust
use rupa_mobile::{bootstrap_mobile, App};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    let app = App::new("Mobile App").root(MyComponent);
    bootstrap_mobile(app);
}
```
