use std::convert::Infallible;
use std::future::Future;
use std::num::NonZeroU32;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::body::Body;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use governor::clock::DefaultClock;
use governor::state::keyed::DashMapStateStore;
use governor::{Quota, RateLimiter};
use tower::{Layer, Service};

type KeyedLimiter = RateLimiter<String, DashMapStateStore<String>, DefaultClock>;
type BoxFuture = Pin<Box<dyn Future<Output = Result<Response, Infallible>> + Send>>;

pub type KeyExtractor = Arc<dyn Fn(&Request<Body>) -> String + Send + Sync>;

#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: Arc<KeyedLimiter>,
    key_extractor: KeyExtractor,
}

impl RateLimitLayer {
    pub fn per_second(requests: u32, burst: u32) -> Self {
        let requests = NonZeroU32::new(requests)
            .expect("rate limit requests per second must be greater than zero");
        let burst = NonZeroU32::new(burst).expect("rate limit burst must be greater than zero");
        let quota = Quota::per_second(requests).allow_burst(burst);
        Self {
            limiter: Arc::new(RateLimiter::keyed(quota)),
            key_extractor: Arc::new(|request| {
                request
                    .extensions()
                    .get::<std::net::SocketAddr>()
                    .map(|addr| format!("rate_ip_{}", addr.ip()))
                    .unwrap_or_else(|| "rate_ip_unknown".to_string())
            }),
        }
    }

    pub fn with_key_extractor(
        mut self,
        key_extractor: impl Fn(&Request<Body>) -> String + Send + Sync + 'static,
    ) -> Self {
        self.key_extractor = Arc::new(key_extractor);
        self
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            limiter: self.limiter.clone(),
            key_extractor: self.key_extractor.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    limiter: Arc<KeyedLimiter>,
    key_extractor: KeyExtractor,
}

impl<S> Service<Request<Body>> for RateLimitService<S>
where
    S: Service<Request<Body>, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future = BoxFuture;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let key = (self.key_extractor)(&request);
        if self.limiter.check_key(&key).is_err() {
            return Box::pin(async {
                Ok(crate::errors::ErrTooManyRequests.clone().into_response())
            });
        }

        let future = self.inner.call(request);
        Box::pin(future)
    }
}
