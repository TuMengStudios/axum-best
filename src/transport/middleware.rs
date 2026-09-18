//! Transport-layer middleware, one submodule per concern.
//!
//! - [`request_id`]: propagate the request id into tracing spans and response headers
//! - [`timeout`]: enforce a request time budget, with per-path exclusions
//! - [`compression`]: compress responses, with per-path exclusions
//! - [`cors`]: configure cross-origin resource sharing
//! - [`prefix`]: the segment-aware path matching shared by the exclusions

pub mod compression;
pub mod cors;
pub mod prefix;
pub mod request_id;
pub mod timeout;
