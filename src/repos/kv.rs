use async_trait::async_trait;
use redis::{FromRedisValue, ToSingleRedisArg};

use crate::core::rest::AppError;

/// KV store trait: abstracts cache storage such as Redis; concrete implementations are provided by the data layer
#[async_trait]
pub trait KvStore: Send + Sync {
    /// Writes a typed value. `redis-macros` types serialize to JSON here.
    async fn set<T>(&self, key: &str, value: T) -> Result<(), AppError>
    where
        T: ToSingleRedisArg + Send + Sync,
        Self: Sized;

    /// Writes a typed value with an expiration time in seconds.
    async fn set_ex<T>(&self, key: &str, value: T, seconds: u64) -> Result<(), AppError>
    where
        T: ToSingleRedisArg + Send + Sync,
        Self: Sized;

    /// Reads the value for a key. Missing keys and deserialization failures are errors.
    async fn get<T>(&self, key: &str) -> Result<T, AppError>
    where
        T: FromRedisValue,
        Self: Sized;

    /// Deletes a key
    async fn del(&self, key: &str) -> Result<(), AppError>;
}
