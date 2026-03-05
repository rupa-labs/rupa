# Core Philosophy

Rupaui is built on two foundational pillars: **Utility-First Flexibility** and **Semantic-Support Structure**. It is designed specifically for high-performance applications that demand an "Artisan" level of control.

## 1. Utility-First, Semantic-Support

Unlike traditional frameworks that force you to choose between messy utility classes or rigid semantic components, Rupaui embraces both.

- **Semantic Components**: Structs like `Button`, `Section`, and `Input` provide meaningful structure and handle complex internal states (A11y, Focus, Validation).
- **Utility Styling**: A fluent, functional API allows you to decorate these components without writing custom CSS or creating dozens of single-use wrapper components.

## 2. Command Buffer Reactive Architecture (CBRA)

Rupaui is designed to work seamlessly with the CBRA model:
- **Intents**: User interactions are normalized into semantic intents.
- **Stateless Execution**: The UI merely reflects the current state of the engine.
- **Signals**: High-performance, fine-grained reactivity ensures that only the necessary parts of the UI are updated when state changes.

## 3. Data-Oriented Design (DOD)

To ensure maximum performance on lower-end hardware and WebAssembly:
- **Flat Data Structures**: Properties are stored in contiguous memory buffers.
- **Zero-Cost Abstractions**: Leveraging Rust's compiler to optimize layout and painting passes.
- **Shader-Ready**: Styling properties are designed to be easily mapped to GPU shader uniforms.
