# Component Reference Index 📚

This document provides a comprehensive list of all UI components available in Rupaui, organized by their functional categories. Use this guide to find the right building blocks for your application.

---

## 🏛️ App Infrastructure
The invisible backbone of every Rupaui application.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Root** | High-level container for App Metadata and orchestration. | [View Detail](./05-architecture/root-and-body.md) |
| **Body** | Implicit root-level container that always fills the window. | [View Detail](./05-architecture/root-and-body.md) |

---

## 🏗️ Layout Containers
Structural elements used to organize and align other components.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **VStack** | Vertical stacking container with gap support. | [View Detail](./07-components/vstack.md) |
| **HStack** | Horizontal stacking container with gap support. | [View Detail](./07-components/hstack.md) |
| **Row** | Alias for HStack. Horizontal alignment. | [View Detail](./07-components/hstack.md) |
| **Col** | Alias for VStack. Vertical alignment. | [View Detail](./07-components/vstack.md) |
| **Container** | Main layout wrapper with constrained max-width. | [View Detail](./07-components/layout.md) |
| **Section** | Semantic grouping for major content blocks. | [View Detail](./07-components/layout.md) |
| **Div** | The most basic box container for general use. | [View Detail](./06-primitives/div.md) |
| **Flex** | A flexible box container for advanced alignments. | [View Detail](./06-primitives/flex.md) |
| **Grid** | A grid-based container for complex 2D layouts. | [View Detail](./06-primitives/grid.md) |

---

## 🏁 Base & Branding
Fundamental elements for content and identity.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Text** | The primary element for all typographic content. | [View Detail](./07-components/text.md) |
| **Brand** | Specialized wrapper for logos and brand identities. | [View Detail](./07-components/brand.md) |
| **Svg** | Container for scalable vector graphics. | [View Detail](./07-components/svg.md) |
| **Icon** | Semantic wrapper for UI icons. | [View Detail](./07-components/svg.md) |

---

## ⚡ Interactive Elements
Components designed for user triggers and actions.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Button** | Standard trigger for actions and events. | [View Detail](./07-components/button.md) |
| **CloseButton** | Specialized red button for closing panels or modals. | [View Detail](./07-components/button.md) |
| **ButtonGroup** | Flex container specifically for grouping multiple buttons. | [View Detail](./07-components/button.md) |
| **ThemeSwitcher** | Ready-to-use toggle for Light and Dark modes. | [View Detail](./07-components/theme-switcher.md) |

---

## 📝 Forms & Inputs
Building blocks for capturing user data and choices.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Label** | Semantic and accessible text for form fields. | [View Detail](./07-components/forms.md) |
| **Input** | Clean field for single-line text entry. | [View Detail](./07-components/forms.md) |
| **Checkbox** | Classic binary toggle for selections. | [View Detail](./07-components/forms.md) |
| **Radio** | Option for mutually exclusive selections. | [View Detail](./07-components/forms.md) |
| **Switch** | Modern toggle for boolean states. | [View Detail](./07-components/forms.md) |
| **Select** | Dropdown menu for picking from a list. | [View Detail](./07-components/forms.md) |

---

## 📦 Content & Data
Containers designed to present grouped or structured information.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Card** | Elevated surface for grouping related content. | [View Detail](./07-components/content.md) |
| **Table** | Grid-based view for structured data sets. | [View Detail](./07-components/content.md) |
| **Accordion** | Space-saving collapsible content sections. | [View Detail](./07-components/content.md) |

---

## 🔔 Feedback & Loading
Indicators that provide status updates or handle waiting states.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Alert** | Important messages and notification banners. | [View Detail](./07-components/feedback.md) |
| **Badge** | Small visual indicator for counts or statuses. | [View Detail](./07-components/feedback.md) |
| **Progress** | Bar showing completion status of a task. | [View Detail](./07-components/feedback.md) |
| **Spinner** | Animated indicator for indeterminate loading. | [View Detail](./07-components/feedback.md) |
| **Skeleton** | Visual placeholder while content is fetching. | [View Detail](./07-components/feedback.md) |

---

## 🧭 Navigation
Tools to help users move through your application.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Navbar** | Standard header for global app navigation. | [View Detail](./07-components/navigation.md) |
| **Tabs** | Navigation for switching between local views. | [View Detail](./07-components/navigation.md) |
| **Breadcrumb** | Path tracking for hierarchical navigation. | [View Detail](./07-components/navigation.md) |

---

## 🖼️ Overlays
Components that appear on top of the main application content.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Modal** | Focused dialog box that blocks background interaction. | [View Detail](./07-components/overlay.md) |
| **Tooltip** | Contextual hint appearing on hover or focus. | [View Detail](./07-components/overlay.md) |
| **Overlay** | Base container for custom floating elements. | [View Detail](./06-primitives/overlay.md) |

---

## ⚙️ Logic & Viewports
Functional components for app structure and conditional flows.

| Component | Description | Documentation |
| :--- | :--- | :--- |
| **Show** | Logic gate for conditional rendering. | [View Detail](./08-composition/control-flow.md) |
| **ForEach** | Efficient rendering for lists and collections. | [View Detail](./08-composition/control-flow.md) |
| **Viewport** | Isolated sub-view with its own layout context. | [View Detail](./08-composition/viewports.md) |
