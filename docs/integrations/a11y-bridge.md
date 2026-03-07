# Module: Accessibility Bridge (A11y) (`src/platform/a11y.rs`) ♿

Accessibility is a foundational component of Rupa Framework, managed as a semantic bridge between the framework and the Operating System's accessibility services.

---

## 🏗️ Architecture: Dependency Inversion

To maintain a clean core, Rupa Framework follows a strict **Dependency Inversion** pattern for accessibility. The framework only defines the *Semantic Intent*, while the platform runners handle the *Implementation Detail* (e.g., AccessKit).

### 🧩 Core Components
1. **SemanticRole:** A platform-agnostic enum representing UI types (`Button`, `Slider`, `Heading`, etc.).
2. **AccessibilityNode:** A metadata container for Screen Readers:
   - `label`: Semantic name (e.g., "Delete File").
   - `description`: Detailed hint.
   - `value`: State string (e.g., "Enabled").
   - `checked` / `expanded`: Interaction flags.

---

## 🌉 Integration Workflow

1. **Component Hook:** Every component implements an `.accessibility()` method returning its `AccessibilityNode`.
2. **Tree Translation:** The Platform Runner traverses the component tree and maps these nodes to OS-specific roles.
3. **Event Notification:** When a component state changes, the platform runner notifies the OS accessibility service to update the screen reader's context.

---

## 🔊 Screen Reader Support

By providing a native accessibility bridge, Rupa Framework applications become accessible via:
- **VoiceOver** (MacOS / iOS)
- **NVDA & JAWS** (Windows)
- **TalkBack** (Android)
- **Orca** (Linux)

---

## 🌟 Comprehensive Accessibility Features

### Semantic Metadata Integration
Rupa Framework components provide structured semantic information to the OS, allowing assistive technologies to understand the purpose and state of each UI element. 
- **Role Awareness:** Components automatically map to OS roles like Buttons, Sliders, and Form Fields.
- **State Synchronization:** Changes in component state (via Signals) are propagated to the OS Accessibility layer, ensuring Screen Readers receive real-time updates.

### Bi-Directional Interaction
The bridge supports action requests from assistive devices, enabling:
- **Remote Activation:** Triggering button clicks or toggles via VoiceOver/NVDA commands.
- **Focus Management:** Controlling application focus directly from accessibility tools.

### Global Compatibility
By implementing a native bridge, Rupa Framework applications are compatible with all major Screen Readers:
- **VoiceOver** (MacOS / iOS)
- **NVDA & JAWS** (Windows)
- **TalkBack** (Android)
- **Orca** (Linux)


