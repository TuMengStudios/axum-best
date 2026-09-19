# errors module

Pre-define global business error code.

- `codes.rs` predefined `AppError` statics for common, auth, Redis, WeChat, and MySQL/SQLx errors
- `mod.rs` re-exports all statics, callers use them via `crate::errors::Err*`
