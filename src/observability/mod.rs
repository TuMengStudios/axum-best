//! OpenTelemetry initialization and request tracing integration.

mod otel;

pub use otel::{OpenTelemetryConfig, OpenTelemetryGuard, init_open_telemetry};
