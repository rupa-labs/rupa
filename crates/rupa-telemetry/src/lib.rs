//! # Rupa Telemetry 📊
//!
//! Observability, Logging, Metrics, and Profiling for the Rupa Framework. 
//! Provides a unified hub (Ports) for framework-wide diagnostic data.

pub mod logger;
pub mod metrics;
pub mod profiler;

use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;

pub use logger::{Logger, Level, ConsoleLogger, NullLogger};
pub use metrics::Recorder;
pub use profiler::Profiler;

/// The central hub for all framework observability.
///
/// Combines structured logging, metrics collection, and execution 
/// profiling into a single thread-safe interface.
pub struct Telemetry {
    /// Pluggable logger adapter.
    pub logger: Arc<dyn Logger>,
    /// Pluggable metrics recorder adapter.
    pub metrics: Arc<dyn Recorder>,
    /// Pluggable profiler adapter.
    pub profiler: Arc<dyn Profiler>,
}

static DEFAULT_TELEMETRY: Lazy<Telemetry> = Lazy::new(|| {
    Telemetry {
        logger: Arc::new(ConsoleLogger),
        metrics: Arc::new(NullRecorder),
        profiler: Arc::new(NullProfiler),
    }
});

impl Telemetry {
    /// Returns the global telemetry instance.
    pub fn global() -> &'static Self {
        &DEFAULT_TELEMETRY
    }

    /// Creates a new telemetry hub with custom adapters.
    pub fn new(
        logger: Arc<dyn Logger>,
        metrics: Arc<dyn Recorder>,
        profiler: Arc<dyn Profiler>,
    ) -> Self {
        Self { logger, metrics, profiler }
    }
}

/// A null implementation of the Recorder trait.
pub struct NullRecorder;
impl Recorder for NullRecorder {
    fn increment(&self, _: &str, _: u64) {}
    fn gauge(&self, _: &str, _: f64) {}
    fn timing(&self, _: &str, _: f64) {}
}

/// A null implementation of the Profiler trait.
pub struct NullProfiler;
impl Profiler for NullProfiler {
    fn start_span(&self, _: &str) {}
    fn end_span(&self, _: &str) {}
}

/// A mock implementation of the Telemetry hub for TDD and headless testing.
pub struct MockTelemetry {
    pub logs: Arc<RwLock<Vec<String>>>,
    pub metrics: Arc<RwLock<Vec<(String, f64)>>>,
}

impl MockTelemetry {
    /// Creates a new, empty mock telemetry instance.
    pub fn new() -> Self {
        Self {
            logs: Arc::new(RwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_telemetry_flow() {
        let mock = MockTelemetry::new();
        mock.logs.write().unwrap().push("Test Log".to_string());
        
        let logs = mock.logs.read().unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0], "Test Log");
    }
}
