use std::time::Duration;

use anyhow::Context;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{Sampler, Tracer, TracerProvider};
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator};
use serde::Deserialize;

const DEFAULT_TRACER_NAME: &str = "axum-best";
const DEFAULT_SAMPLE_RATIO: f64 = 0.05;

/// OpenTelemetry export configuration.
#[derive(Debug, Deserialize)]
pub struct OpenTelemetryConfig {
    /// OTLP gRPC endpoint. When omitted, spans stay local and are written by tracing-subscriber.
    #[serde(default)]
    pub endpoint: Option<String>,

    /// Name used when creating the OpenTelemetry tracer.
    #[serde(default)]
    pub tracer_name: Option<String>,

    /// Fraction of traces to sample. Must be between 0.0 and 1.0.
    #[serde(default = "default_sample_ratio")]
    pub sample_ratio: f64,
}

impl Default for OpenTelemetryConfig {
    fn default() -> Self {
        Self {
            endpoint: None,
            tracer_name: None,
            sample_ratio: DEFAULT_SAMPLE_RATIO,
        }
    }
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
        let sample_ratio = Self::validate_sample_ratio(self.sample_ratio)
            .context("invalid OpenTelemetry sample ratio")?;
        let trace_config = || {
            let config =
                opentelemetry_sdk::trace::Config::default().with_resource(Resource::new(vec![
                    opentelemetry::KeyValue::new("service.name", service_name.clone()),
                ]));
            config.with_sampler(Sampler::TraceIdRatioBased(sample_ratio))
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

    fn validate_sample_ratio(ratio: f64) -> anyhow::Result<f64> {
        anyhow::ensure!(ratio.is_finite() && (0.0..=1.0).contains(&ratio));
        Ok(ratio)
    }

    fn resolved_tracer_name(&self) -> String {
        self.tracer_name
            .as_deref()
            .filter(|name| !name.trim().is_empty())
            .unwrap_or(DEFAULT_TRACER_NAME)
            .to_string()
    }
}

fn default_sample_ratio() -> f64 {
    DEFAULT_SAMPLE_RATIO
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
    use super::{DEFAULT_SAMPLE_RATIO, DEFAULT_TRACER_NAME, OpenTelemetryConfig};

    #[test]
    fn sample_ratio_defaults_to_five_percent() {
        assert_eq!(OpenTelemetryConfig::default().sample_ratio, DEFAULT_SAMPLE_RATIO);
    }

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

    #[test]
    fn sample_ratio_accepts_values_between_zero_and_one() {
        assert_eq!(OpenTelemetryConfig::validate_sample_ratio(0.0).unwrap(), 0.0);
        assert_eq!(OpenTelemetryConfig::validate_sample_ratio(0.25).unwrap(), 0.25);
        assert_eq!(OpenTelemetryConfig::validate_sample_ratio(1.0).unwrap(), 1.0);
    }

    #[test]
    fn sample_ratio_rejects_invalid_values() {
        for ratio in [-0.1, 1.1, f64::NAN, f64::INFINITY] {
            assert!(OpenTelemetryConfig::validate_sample_ratio(ratio).is_err());
        }
    }
}
