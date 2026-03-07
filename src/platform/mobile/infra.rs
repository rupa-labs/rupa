/// Wrapper for Mobile-specific infrastructure.
/// Adheres to Dependency Inversion by isolating native glue logic.
pub struct MobileInfra;

impl MobileInfra {
    /// Provides platform-specific hints for the renderer.
    /// Useful for Notch handling (Safe Area) and high-refresh rates.
    pub fn get_safe_area_insets() -> (f32, f32, f32, f32) {
        // TODO: Bridge to JNI (Android) or CoreGraphics (iOS)
        (0.0, 0.0, 0.0, 0.0)
    }

    pub fn set_orientation_locked(locked: bool) {
        log::info!("Rupaui: Orientation lock set to: {}", locked);
        // TODO: Native orientation lock
    }
}
