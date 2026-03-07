# Platform Integration: Mobile Runner (`mobile/runner.rs`) 📱

The **Mobile Runner** provides the execution shell for native mobile applications on Android and iOS. It manages the unique lifecycle constraints of mobile operating systems, such as background suspension and resource reclamation.

---

## 🏗️ Architecture

The Mobile Runner bridges native OS events (via `winit` and `MobileInfra`) into the Rupa Framework ecosystem.

### Key Responsibilities
- **Lifecycle Management**: Responds to `suspended` and `resumed` events to destroy/recreate GPU surfaces.
- **Surface Invalidation**: Automatically handles surface loss when the app moves to the background.
- **Safe Area Insets**: Provides information for the renderer to avoid drawing under notches or home indicators.
- **Touch & Gesture**: Maps native multi-touch events into Rupa Framework's pointer system.

---

## 🗝️ API & Usage

### Starting a Mobile App
Mobile applications are typically started via the standard `App::run_mobile()` method, which prepares the runner for native mobile constraints:

```rust
App::new("My Mobile App")
    .run_mobile();
```

### Execution Flow
1. **Resumed**: Creates the OS window and initializes the GPU renderer.
2. **Suspended**: **Destroys the renderer and surface** to release VRAM, as required by mobile OS guidelines.
3. **Redraw**: Handles the frame-by-frame rendering loop when the app is active and visible.

---

## 🛡️ Dependency Inversion (MobileInfra)

All native glue logic (JNI for Android, CoreGraphics for iOS) is abstracted behind `MobileInfra` (`src/platform/mobile/infra.rs`). This prevents native mobile platform types from leaking into the agnostik Platform Integration runner logic.
