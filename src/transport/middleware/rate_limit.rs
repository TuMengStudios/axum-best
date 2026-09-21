use std::convert::Infallible;
use std::future::Future;
use std::num::NonZeroU32;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use axum::body::Body;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use governor::clock::DefaultClock;
use governor::state::keyed::DashMapStateStore;
use governor::{Quota, RateLimiter};
use tower::{Layer, Service};

use crate::auth::Claims;

type KeyedLimiter = RateLimiter<String, DashMapStateStore<String>, DefaultClock>;
type BoxFuture = Pin<Box<dyn Future<Output = Result<Response, Infallible>> + Send>>;

pub type KeyExtractor = Arc<dyn Fn(&Request<Body>) -> String + Send + Sync>;

#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: Arc<KeyedLimiter>,
    key_extractor: KeyExtractor,
}

impl RateLimitLayer {
    /// Creates a rate-limit layer with a quota over the given period.
    ///
    /// # Arguments
    ///
    /// * `period` - The time window used to calculate the average request rate.
    /// * `requests` - The number of requests allowed during `period`.
    /// * `burst` - The maximum number of requests that can pass in a short burst
    ///   before the average rate limit is enforced.
    ///
    /// For example, `period = 1 second`, `requests = 10`, and `burst = 10`
    /// allows an average of 10 requests per second with an initial burst of 10.
    pub fn with_quota(period: Duration, requests: u32, burst: u32) -> Self {
        assert!(!period.is_zero(), "rate limit period must be greater than zero");
        let requests =
            NonZeroU32::new(requests).expect("rate limit requests must be greater than zero");
        let burst = NonZeroU32::new(burst).expect("rate limit burst must be greater than zero");
        let interval = period
            .checked_div(requests.get())
            .expect("rate limit period is too short for the request count");
        let quota = Quota::with_period(interval)
            .expect("rate limit period is too short for the request count")
            .allow_burst(burst);
        Self {
            limiter: Arc::new(RateLimiter::keyed(quota)),
            key_extractor: Arc::new(|request| {
                let path = request.uri().path().replace('/', "_");
                request
                    .extensions()
                    .get::<std::net::SocketAddr>()
                    .map(|addr| {
                        let ip = addr.ip().to_string().replace('.', "_");
                        format!("rate_ip_{}_{}", ip, path)
                    })
                    .unwrap_or_else(|| format!("rate_ip_unknown_{}", path))
            }),
        }
    }

    /// Creates a rate-limit layer keyed by the authenticated user's ID and path.
    pub fn with_login_quota(period: Duration, requests: u32, burst: u32) -> Self {
        Self::with_quota(period, requests, burst).with_key_extractor(Self::user_key)
    }

    fn user_key(request: &Request<Body>) -> String {
        let path = request.uri().path().replace('/', "_");
        let user_id = request
            .extensions()
            .get::<Claims>()
            .map(|claims| claims.user_id.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        format!("rate_user_{}_{}", path, user_id)
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
