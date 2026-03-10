pub mod bridge;
pub mod translate;

pub use bridge::A11yBridge;

/// Bridges the framework's internal accessibility nodes to the OS.
pub fn bridge_to_os() {
    todo!("Accessibility bridge implementation pending integration with AccessKit")
}
