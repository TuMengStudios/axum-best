use async_trait::async_trait;

use crate::core::rest::AppError;

/// KV store trait: abstracts cache storage such as Redis; concrete implementations are provided by the data layer
#[async_trait]
pub trait KvStore: Send + Sync {
    /// Writes a key/value pair
    async fn set(&self, key: &str, value: &str) -> Result<(), AppError>;

    /// Reads the value for a key, returning None when the key does not exist
    async fn get(&self, key: &str) -> Result<Option<String>, AppError>;

    /// Deletes a key
    async fn del(&self, key: &str) -> Result<(), AppError>;
}
