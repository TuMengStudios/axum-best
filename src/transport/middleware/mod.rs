//! Transport-layer middleware, one submodule per concern.
//!
//! - [`timeout`]: enforce a request time budget, with per-path exclusions
//! - [`compression`]: compress responses, with per-path exclusions
//! - [`cors`]: configure cross-origin resource sharing
//! - [`auth`]: validate JWT bearer credentials
//! - [`prefix`]: the segment-aware path matching shared by the exclusions

pub mod auth;
pub mod compression;
pub mod cors;
pub mod otel;
pub mod prefix;
pub mod timeout;
