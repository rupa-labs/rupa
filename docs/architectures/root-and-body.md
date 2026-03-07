# Infrastructure: Root & Body 🏛️

The Root and Body elements are the foundational layers of every Rupa Framework application. They provide the transition from the host Operating System into the Rupa Framework agnostik ecosystem.

---

## 🏗️ The Root (App Metadata Manifest)

The **Root** is managed by the `App` struct. It is not a visual element but an architectural manifest that synchronizes the application's identity with the host environment.

### 1. Metadata Manifest
Rupa Framework supports modern metadata standards (similar to Web PWAs), allowing you to define the app's personality beyond just a title.

| Property | Description | Platform Sync |
| :--- | :--- | :--- |
| **Title** | The display name of the app. | Window title / document.title |
| **Icon** | App logo (Path or Bytes). | Window icon / Favicon |
| **Theme Color** | Primary brand color. | Browser theme-color / Status bar |
| **Display Mode** | Viewing preference. | Fullscreen / Standalone (Web) |

---

## 🕺 The Body (Universal Container & Z-Stack)

The **Body** is the internal root-level visual component (`src/core/body.rs`). It acts as the ultimate viewport and manages the **Global Z-Stack**, **Viewport Context**, and **Interaction Isolation**.

### 1. The Multi-Layer Architecture
To handle complex UI scenarios like Modals and Tooltips, the Body maintains a dual-layer system:
- **Primary Content Layer**: Where your main application tree lives.
- **Global Overlay Layer**: A prioritized layer for elements that must appear above all content.

### 2. Interaction Isolation (Focus Trapping & Backdrop)
When an overlay is designated as a **Modal**, the Body enforces strict interaction rules:
- **Backdrop (Dimmer)**: An optional semi-transparent layer is rendered between the Primary Content and the Modal to visually and functionally isolate the content.
- **Input Blocking**: All pointer events (clicks, scrolls) are blocked from reaching the Primary Content layer.
- **Focus Trap**: Keyboard navigation (e.g., `Tab` key) is restricted to the components within the active Modal. Focus cannot "leak" back into the background content.

### 3. Global Viewport Context
The Body maintains a reactive **Viewport Signal**. This allows any component in the tree to listen to window size changes and adapt its layout (e.g., switching from a Sidebar to a Bottom Nav on mobile).

### 4. Pointer Cursor Management
Rupa Framework supports context-aware mouse cursors. 
- **Resolution**: During the hit-test phase, the `InputDispatcher` identifies the topmost hovered component and reads its `cursor` style.
- **Feedback Loop**: The dispatcher sends a request back to the **Platform Integration Runner** to update the native OS cursor.

---

## 🔄 Interaction Flow

1.  **Isolation Check**: Before dispatching any event, the `InputDispatcher` checks if a "Focus Trap" is active.
2.  **Hit-Testing Priority**: If a trap is active, only the active overlay is searched for targets. Otherwise, the Overlay Layer has priority over the Content Layer.
3.  **Cursor Resolution**: Determins the requested cursor shape from the hit component's style.
4.  **Paint Order**: Content -> Optional Backdrop -> Overlays.
