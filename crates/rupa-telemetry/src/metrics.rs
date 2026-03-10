/// Collection of performance metrics.
#[derive(Default)]
pub struct Metrics {
    pub fps: f32,
    pub render_time_ms: f32,
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }
}
