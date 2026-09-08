use async_trait::async_trait;
use redis::AsyncCommands;
use tracing::error;

use crate::core::rest::AppError;
use crate::data::cache::RedisPool;
use crate::errors;
use crate::repos::kv::KvStore;

/// 基于 Redis（bb8 异步连接池）的 KV 存储实现
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
    /// 写入 key/value
    async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        let mut conn = self.pool.get().await.map_err(|err| {
            error!("get redis connection error {}", err);
            errors::ErrRedisClient.clone()
        })?;
        let _: () = conn.set(key, value).await.map_err(|err| {
            error!("set redis key error {}", err);
            errors::ErrRedisClient.clone()
        })?;
        Ok(())
    }

    /// 读取 key 对应的值，key 不存在时返回 None
    async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self.pool.get().await.map_err(|err| {
            error!("get redis connection error {}", err);
            errors::ErrRedisClient.clone()
        })?;
        let value: Option<String> = conn.get(key).await.map_err(|err| {
            error!("get redis key error {}", err);
            errors::ErrRedisClient.clone()
        })?;
        Ok(value)
    }

    /// 删除 key
    async fn del(&self, key: &str) -> Result<(), AppError> {
        let mut conn = self.pool.get().await.map_err(|err| {
            error!("get redis connection error {}", err);
            errors::ErrRedisClient.clone()
        })?;
        let _: () = conn.del(key).await.map_err(|err| {
            error!("del redis key error {}", err);
            errors::ErrRedisClient.clone()
        })?;
        Ok(())
    }
}
