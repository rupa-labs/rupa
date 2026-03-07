use rupa_core::error::{Error, Result};

/// Wrapper for Mobile-specific infrastructure.
/// Adheres to Dependency Inversion by isolating native glue logic.
pub struct MobileInfra;

impl MobileInfra {
    /// Provides platform-specific hints for the renderer.
    /// Useful for Notch handling (Safe Area) and high-refresh rates.
    pub fn get_safe_area_insets() -> (f32, f32, f32, f32) {
        // TODO: Bridge to JNI (Android) or CoreGraphics (iOS)
        // For now, return zero insets as a safe default but log it if needed
        (0.0, 0.0, 0.0, 0.0)
    }

    pub fn set_orientation_locked(locked: bool) -> Result<()> {
        log::info!("Rupa: Orientation lock set to: {}", locked);
        Err(Error::Unsupported("Native orientation lock".into()))
    }
}
