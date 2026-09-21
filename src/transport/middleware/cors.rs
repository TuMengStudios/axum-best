//! Cross-origin resource sharing middleware configuration.

use tower_http::cors::CorsLayer;
use tower_http::cors::{AllowOrigin, Any};

/// Build the application's CORS layer.
pub fn layer(origins: &[String]) -> CorsLayer {
    if origins.is_empty() {
        return CorsLayer::new();
    }

    if origins.iter().any(|origin| origin == "*") {
        return CorsLayer::new().allow_origin(Any);
    }

    let origins = origins.to_vec();
    CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin, _| {
            origins
                .iter()
                .any(|allowed| allowed.as_bytes() == origin.as_bytes())
        }))
        .allow_credentials(true)
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use tower::{ServiceBuilder, ServiceExt, service_fn};

    use super::layer;

    async fn response(origins: &[String], request_origin: &str) -> axum::response::Response {
        ServiceBuilder::new()
            .layer(layer(origins))
            .service(service_fn(|_| async {
                Ok::<_, std::convert::Infallible>(axum::response::Response::new(Body::empty()))
            }))
            .oneshot(
                Request::builder()
                    .header("origin", request_origin)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn only_configured_origins_receive_cors_headers() {
        let origins = vec!["https://trusted.example".to_owned()];
        let resp = response(&origins, "https://trusted.example").await;
        assert_eq!(
            resp.headers().get("access-control-allow-origin").unwrap(),
            "https://trusted.example"
        );
        assert_eq!(
            resp.headers()
                .get("access-control-allow-credentials")
                .unwrap(),
            "true"
        );

        let resp = response(&origins, "https://untrusted.example").await;
        assert!(resp.headers().get("access-control-allow-origin").is_none());
    }

    #[tokio::test]
    async fn wildcard_allows_any_origin_without_credentials() {
        let origins = vec!["*".to_owned()];
        let resp = response(&origins, "https://any.example").await;
        assert_eq!(resp.headers().get("access-control-allow-origin").unwrap(), "*");
        assert!(
            resp.headers()
                .get("access-control-allow-credentials")
                .is_none()
        );
    }

    #[tokio::test]
    async fn missing_origins_disables_cors() {
        let resp = response(&[], "https://any.example").await;
        assert!(resp.headers().get("access-control-allow-origin").is_none());
        assert!(
            resp.headers()
                .get("access-control-allow-credentials")
                .is_none()
        );
    }
}
