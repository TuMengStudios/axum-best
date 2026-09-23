use crate::core::rest::AppError;
use crate::data::cache::RedisPool;
use crate::errors;
use crate::repos::kv::KvStore;
use async_trait::async_trait;
use redis::AsyncCommands;

/// KV store implementation backed by Redis (bb8 async connection pool)
pub struct RedisKvStore {
    pool: RedisPool,
}

impl RedisKvStore {
    pub fn new(pool: RedisPool) -> RedisKvStore {
        RedisKvStore { pool }
    }
}

#[async_trait]
impl KvStore for RedisKvStore {
    /// Writes a key/value pair
    async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|err| errors::ErrRedisClient.with_cause(err, "get redis connection"))?;
        let _: () = conn
            .set(key, value)
            .await
            .map_err(|err| errors::ErrRedisClient.with_cause(err, "set redis key"))?;
        Ok(())
    }

    /// Reads the value for a key, returning None when the key does not exist
    async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|err| errors::ErrRedisClient.with_cause(err, "get redis connection"))?;
        let value: Option<String> = conn
            .get(key)
            .await
            .map_err(|err| errors::ErrRedisClient.with_cause(err, "get redis key"))?;
        Ok(value)
    }

    /// Deletes a key
    async fn del(&self, key: &str) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|err| errors::ErrRedisClient.with_cause(err, "get redis connection"))?;
        let _: () = conn
            .del(key)
            .await
            .map_err(|err| errors::ErrRedisClient.with_cause(err, "delete redis key"))?;
        Ok(())
    }
}
