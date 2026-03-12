pub mod logger;
pub mod metrics;
pub mod profiler;

use std::sync::Arc;
use once_cell::sync::Lazy;

pub use logger::{Logger, Level, ConsoleLogger, NullLogger};
pub use metrics::Recorder;
pub use profiler::Profiler;

/// The central hub for all framework observability.
pub struct Telemetry {
    pub logger: Arc<dyn Logger>,
    pub metrics: Arc<dyn Recorder>,
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
    pub fn global() -> &'static Self {
        &DEFAULT_TELEMETRY
    }

    pub fn new(
        logger: Arc<dyn Logger>,
        metrics: Arc<dyn Recorder>,
        profiler: Arc<dyn Profiler>,
    ) -> Self {
        Self { logger, metrics, profiler }
    }
}

pub struct NullRecorder;
impl Recorder for NullRecorder {
    fn increment(&self, _: &str, _: u64) {}
    fn gauge(&self, _: &str, _: f64) {}
    fn timing(&self, _: &str, _: f64) {}
}

pub struct NullProfiler;
impl Profiler for NullProfiler {
    fn start_span(&self, _: &str) {}
    fn end_span(&self, _: &str) {}
}
