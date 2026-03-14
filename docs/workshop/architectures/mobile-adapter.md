# Mobile Adapter Architecture 📱

The **Mobile Adapter** is a Tier 3 Showroom that provides native integration for iOS and Android devices.

---

## 1. Technical Context

- **Android**: Uses **Vulkan** for graphics and **JNI** for native system communication.
- **iOS**: Uses **Metal** for graphics and **Swift/Objective-C** bridges for native APIs.

---

## 2. Core Responsibilities

- **Touch Interaction**: Specialized mapping for multi-touch gestures.
- **SafeArea Management**: Adapting layouts to notches and system bars.
- **Native Bridges**: Accessing mobile-specific hardware (Camera, GPS, Notifications).
