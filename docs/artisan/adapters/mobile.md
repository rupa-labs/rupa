# Mobile Adapter 📱

The **Mobile Adapter** provides the native manifestation of Rupa applications for iOS and Android devices.

---

## 🏗️ Technical Architecture
- **Android Runtime**: Uses **Vulkan** for high-performance graphics and the **Android NDK** for native system access.
- **iOS Runtime**: Uses **Metal** for graphics and native **Swift** bridges for system integration.
- **Gesture Engine**: Normalizes complex touch interactions (Swipe, Pinch, Rotation) into Rupa semantic intents.

---

## 🎨 Artisan Focus
Designed for **Portable Excellence**. It allows you to deliver the performance of a native mobile app while sharing the majority of your UI logic with Desktop and Web. Perfect for building fluid, touch-optimized interfaces that feel truly native.

---

## 🗝️ Standard Workflow
1.  **Build**: Use the Rupa CLI to package the masterpiece for mobile (APK or IPA).
2.  **Manifestation**: The adapter handles the native window surface and safe area insets.
3.  **Optimization**: Leverages mobile-specific GPU features for energy-efficient rendering.

[Explore Mobile Technical Spec](../../workshop/architectures/mobile-adapter.md)
