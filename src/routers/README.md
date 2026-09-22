# routers module

Construct web application router, one submodule per route domain

- `mod.rs` merges the route modules, applies the global middleware stack and the optional metrics endpoint
- `user.rs` all `/user/*` routes, auth-guarded except the WeChat login
- `foo.rs` demo `/foo` route behind the auth middleware
- `health.rs` public health check
- `metrics.rs` optional Prometheus metrics endpoint
- `layers.rs` the global middleware stack (tracing, otel, compression, timeout, body limit, CORS)
