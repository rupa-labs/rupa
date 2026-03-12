# `rupa-telemetry` 🩺

**The Observability Port.** Agnostic logging, metrics, and profiling for the framework.

## 🛠️ Key Features

- **`Logger`**: Port for structured application logs.
- **`Recorder`**: Port for performance metrics (Counters, Gauges, Timings).
- **`Profiler`**: Port for execution span profiling.
- **`Telemetry`**: Central hub for all observability adapters.

## 🚀 Usage

```rust
use rupa_telemetry::{Level, Telemetry};

// 1. Log events
telemetry.logger.log(Level::Info, "renderer", "Frame rendered in 16ms");

// 2. Record metrics
telemetry.metrics.gauge("fps", 60.0);

// 3. Profile spans
telemetry.profiler.start_span("layout_recalculation");
// ... compute ...
telemetry.profiler.end_span("layout_recalculation");
```
