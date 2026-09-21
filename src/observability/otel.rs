use std::time::Duration;

use anyhow::Context;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{Tracer, TracerProvider};
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator};
use serde::Deserialize;

const DEFAULT_TRACER_NAME: &str = "axum-best";

/// OpenTelemetry export configuration.
#[derive(Debug, Default, Deserialize)]
pub struct OpenTelemetryConfig {
    /// OTLP gRPC endpoint. When omitted, spans stay local and are written by tracing-subscriber.
    #[serde(default)]
    pub endpoint: Option<String>,

    /// Name used when creating the OpenTelemetry tracer.
    #[serde(default)]
    pub tracer_name: Option<String>,
}

/// Owns the OpenTelemetry provider and flushes spans when dropped.
pub struct OpenTelemetryGuard {
    provider: TracerProvider,
    tracer_name: String,
}

impl OpenTelemetryGuard {
    pub(crate) fn tracer(&self) -> Tracer {
        self.provider.tracer(self.tracer_name.clone())
    }

    /// Flushes and shuts down the exporter. Calling this method more than once is harmless.
    pub fn shutdown(self) {
        let _ = self.provider.shutdown();
    }
}

impl Drop for OpenTelemetryGuard {
    fn drop(&mut self) {
        let _ = self.provider.shutdown();
    }
}

impl OpenTelemetryConfig {
    fn initialize(&self, service_name: impl Into<String>) -> anyhow::Result<OpenTelemetryGuard> {
        global::set_text_map_propagator(TraceContextPropagator::new());
        let service_name = service_name.into();
        let trace_config = || {
            opentelemetry_sdk::trace::Config::default().with_resource(Resource::new(vec![
                opentelemetry::KeyValue::new("service.name", service_name.clone()),
            ]))
        };
        let provider = if let Some(endpoint) = self
            .endpoint
            .clone()
            .filter(|endpoint| !endpoint.trim().is_empty())
        {
            let exporter = opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint)
                .with_timeout(Duration::from_secs(10));
            opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(exporter)
                .with_trace_config(trace_config())
                .install_batch(opentelemetry_sdk::runtime::Tokio)
                .context("initialize OpenTelemetry OTLP exporter")?
        } else {
            TracerProvider::builder()
                .with_config(trace_config())
                .build()
        };

        global::set_tracer_provider(provider.clone());

        Ok(OpenTelemetryGuard {
            provider,
            tracer_name: self.resolved_tracer_name(),
        })
    }

    fn resolved_tracer_name(&self) -> String {
        self.tracer_name
            .as_deref()
            .filter(|name| !name.trim().is_empty())
            .unwrap_or(DEFAULT_TRACER_NAME)
            .to_string()
    }
}

/// Initializes OpenTelemetry from application configuration.
pub fn init_open_telemetry(
    service_name: impl Into<String>,
    config: &OpenTelemetryConfig,
) -> anyhow::Result<OpenTelemetryGuard> {
    config.initialize(service_name)
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_TRACER_NAME, OpenTelemetryConfig};

    #[test]
    fn tracer_name_uses_default_when_missing_or_empty() {
        assert_eq!(OpenTelemetryConfig::default().resolved_tracer_name(), DEFAULT_TRACER_NAME);
        assert_eq!(
            OpenTelemetryConfig {
                tracer_name: Some("   ".to_string()),
                ..Default::default()
            }
            .resolved_tracer_name(),
            DEFAULT_TRACER_NAME
        );
    }

    #[test]
    fn tracer_name_uses_configured_value() {
        assert_eq!(
            OpenTelemetryConfig {
                tracer_name: Some("api".to_string()),
                ..Default::default()
            }
            .resolved_tracer_name(),
            "api"
        );
    }
}
