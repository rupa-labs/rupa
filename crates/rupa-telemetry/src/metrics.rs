/// A Port for recording performance metrics.
pub trait Recorder: Send + Sync {
    /// Increments a counter by a given amount.
    fn increment(&self, name: &str, value: u64);
    
    /// Records a specific value at a point in time (e.g. FPS).
    fn gauge(&self, name: &str, value: f64);
    
    /// Records a timing duration (e.g. render time).
    fn timing(&self, name: &str, duration_ms: f64);
}
