/// A Port for execution profiling.
pub trait Profiler: Send + Sync {
    /// Starts a named profiling span.
    fn start_span(&self, name: &str);
    
    /// Ends the current span.
    fn end_span(&self, name: &str);
}
