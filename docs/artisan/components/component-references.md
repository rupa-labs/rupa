# Component Reference Index 📚

This document provides a comprehensive list of all UI components available in Rupa Framework, organized by their functional categories. Use this guide to find the right building blocks for your application.

---

## 🏛️ App Infrastructure
The invisible backbone of every Rupa Framework application.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Root** | High-level container for App Metadata and orchestration. | [View Detail](./architectures/root-and-body.md) |
| **Body** | Implicit root-level container that always fills the window. | [View Detail](./architectures/root-and-body.md) |

---

## 🏗️ Layout Containers
Structural elements used to organize and align other components.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **VStack** | Vertical stacking container with gap support. | [View Detail](./components/vstack.md) |
| **HStack** | Horizontal stacking container with gap support. | [View Detail](./components/hstack.md) |
| **Row** | Semantic alias for HStack. | [View Detail](./components/hstack.md) |
| **Col** | Semantic alias for VStack. | [View Detail](./components/vstack.md) |
| **Container** | Main layout wrapper with constrained max-width. | [View Detail](./components/layout.md) |
| **Section** | Semantic grouping for major content blocks. | [View Detail](./components/layout.md) |
| **Div** | The most basic box container for general use. | [View Detail](./components/div.md) |
| **Fieldset**| A container with a labeled border, common in TUI. | [View Detail](./components/layout.md) |
| **Flex** | A flexible box container for advanced alignments. | [View Detail](./primitives/flex.md) |
| **Grid** | A grid-based container for complex 2D layouts. | [View Detail](./components/grid.md) |

---

## 🏁 Base & Branding
Fundamental elements for content and identity.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Text** | The primary element for all typographic content. | [View Detail](./components/text.md) |
| **Brand** | Specialized wrapper for logos and brand identities. | [View Detail](./components/brand.md) |
| **Svg** | Container for scalable vector graphics. | [View Detail](./components/svg.md) |
| **Icon** | Semantic wrapper for UI icons. | [View Detail](./components/svg.md) |

---

## ⚡ Interactive Elements
Components designed for user triggers and actions.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Button** | Standard trigger for actions and events. | [View Detail](./components/button.md) |
| **CloseButton** | Specialized red button for closing panels or modals. | [View Detail](./components/button.md) |
| **ButtonGroup** | Flex container specifically for grouping multiple buttons. | [View Detail](./components/button.md) |
| **ThemeSwitcher** | Ready-to-use toggle for Light and Dark modes. | [View Detail](./components/theme-switcher.md) |

---

## 📝 Forms & Inputs
Building blocks for capturing user data and choices.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Label** | Semantic and accessible text for form fields. | [View Detail](./components/forms.md) |
| **Input** | Clean field for single-line text entry. | [View Detail](./components/forms.md) |
| **Checkbox** | Classic binary toggle for selections. | [View Detail](./components/forms.md) |
| **Radio** | Option for mutually exclusive selections. | [View Detail](./components/forms.md) |
| **Switch** | Modern toggle for boolean states. | [View Detail](./components/forms.md) |
| **Select** | Dropdown menu for picking from a list. | [View Detail](./components/forms.md) |

---

## 📦 Content & Data
Containers designed to present grouped or structured information.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Card** | Elevated surface for grouping related content. | [View Detail](./components/content.md) |
| **Table** | Grid-based view for structured data sets. | [View Detail](./components/content.md) |
| **Accordion** | Space-saving collapsible content sections. | [View Detail](./components/content.md) |

---

## 🔔 Feedback & Loading
Indicators that provide status updates or handle waiting states.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Alert** | Important messages and notification banners. | [View Detail](./components/feedback.md) |
| **Badge** | Small visual indicator for counts or statuses. | [View Detail](./components/feedback.md) |
| **Progress** | Bar showing completion status of a task. | [View Detail](./components/feedback.md) |
| **Spinner** | Animated indicator for indeterminate loading. | [View Detail](./components/feedback.md) |
| **Skeleton** | Visual placeholder while content is fetching. | [View Detail](./components/feedback.md) |

---

## 🧭 Navigation
Tools to help users move through your application.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Navbar** | Standard header for global app navigation. | [View Detail](./components/navigation.md) |
| **Tabs** | Navigation for switching between local views. | [View Detail](./components/navigation.md) |
| **Breadcrumb** | Path tracking for hierarchical navigation. | [View Detail](./components/navigation.md) |

---

## 🖼️ Overlays
Components that appear on top of the main application content.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Modal** | Focused dialog box that blocks background interaction. | [View Detail](./components/overlay.md) |
| **Tooltip** | Contextual hint appearing on hover or focus. | [View Detail](./components/overlay.md) |
| **Overlay** | Base container for custom floating elements. | [View Detail](./primitives/overlay.md) |

---

## ⚙️ Reactive Logic
Functional components for dynamic tree construction.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Show** | Conditional VNode rendering based on a Signal. | [View Detail](./compositions/control-flow.md) |
| **ForEach** | Key-based list reconciliation for reactive collections. | [View Detail](./compositions/control-flow.md) |
| **Fragment**| Transparent wrapper for returning multiple VNodes. | [View Detail](./architectures/vnode.md) |
