# Module: Headless Environment (`crates/rupa-test/src/headless.rs`) ☁️

This module provides a virtual environment for running Rupa components without graphical output.

---

## 🏗️ Capabilities
- **VNode Capture**: Captures the result of a component's `render()` method.
- **Mock Timers**: Simulates the passage of time for testing async effects and animations.
- **Platform Mocking**: Provides a fake `Renderer` that records draw calls instead of executing them.
