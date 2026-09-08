//! Transport-layer middleware, one submodule per concern.
//!
//! - [`request_id`]: propagate the request id into tracing spans and response headers
//! - [`timeout`]: enforce a request time budget, with per-path exemptions

pub mod request_id;
pub mod timeout;
