# Platform Integration: Mobile Runner 📱

The **Mobile Runner** (provided by `rupa-mobile`) is the high-level composite crate that manages the execution of Rupa Framework applications on **Android** and **iOS**. It bridges native mobile lifecycles and touch interactions into the Rupa core.

---

## 🏗️ Architecture

The mobile implementation is divided into two layers:
1.  **The Shell (`rupa-mobile`):** Manages the native `Activity` (Android) or `UIViewController` (iOS).
2.  **The Runtime (`rupa-engine`):** Handles WGPU-based rendering inside the shell provided by the mobile runner.

### Key Responsibilities
- **Lifecycle Management**: Responds to `suspended` and `resumed` events to manage GPU surface lifetime.
- **Safe Area Propagation**: Communicates notch and home-indicator insets to the UI layout engine.
- **Touch & Gesture Mapping**: Converts native multi-touch events into `InputEvent::Pointer` for the `InputDispatcher`.
- **Native Interop**: Provides safe Rust wrappers for mobile-specific APIs (Camera, GPS, Vibration).

---

## 🗝️ Lifecycle & Rendering

Mobile apps have a "Volatile Surface" policy. Unlike Desktop, the GPU surface can be reclaimed by the OS at any time.

1.  **Resumed**: The app creates/re-acquires the native window handle. `rupa-engine` initializes the WGPU surface.
2.  **Suspended**: The app must drop the WGPU surface and all associated swapchain resources immediately to prevent being killed by the OS.
3.  **RedrawRequested**: Standard rendering loop, optimized for battery preservation.

---

## 🛠️ Mobile-Specific Input

`rupa-mobile` extends the base `InputDispatcher` with touch-specific logic:
- **TouchStart/Move/End**: Mapped to Pointer events.
- **Multi-Touch**: Future support for pinch-to-zoom and rotation gestures.
- **Soft Keyboard**: Logic for resizing the viewport when the virtual keyboard appears.

---

## 🤝 Platform Bridge

All platform-specific code (JNI for Android, Swift/Obj-C bridge for iOS) is isolated within `rupa-mobile`'s internal `infra` module, ensuring that `rupa-core` and `rupa-ui` remain 100% platform-agnostic.
