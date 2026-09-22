use std::time::Duration;

use axum::Router;
use axum::routing::get;
use axum_prometheus::PrometheusMetricLayer;

use crate::core::state::AppState;
use crate::transport::middleware::rate_limit::RateLimitLayer;

/// Adds the Prometheus metrics endpoint when `metrics.path` is configured.
///
/// The route is registered on top of the finished router, after the global
/// layers, so it is intentionally not covered by CORS/timeout/compression
/// etc. The `prometheus_layer` wraps every route to count requests.
pub fn apply(router: Router<AppState>, state: &AppState) -> Router<AppState> {
    match state.cfg.metrics.path() {
        Some(path) => {
            let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
            router
                .route(
                    path,
                    get(move || async move { metric_handle.render() })
                        .layer(RateLimitLayer::with_quota(Duration::from_secs(10), 5, 5)),
                )
                .layer(prometheus_layer)
        }
        None => router,
    }
}
