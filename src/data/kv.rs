use crate::core::rest::AppError;
use crate::data::cache::RedisPool;
use crate::errors;
#[rustfmt::skip]
use bb8::{PooledConnection};
use redis::{AsyncCommands, FromRedisValue, ToSingleRedisArg};

/// KV store implementation backed by Redis (bb8 async connection pool)
///
/// Typed values can use `redis-macros` to store one JSON string per Redis key:
///
/// ```rust,ignore
/// use redis_macros::{FromRedisValue, ToRedisArgs};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
/// struct Session {
///     user_id: i64,
/// }
///
/// redis_store.set("session:42", Session { user_id: 42 }).await?;
/// ```
pub struct RedisKvStore {
    pool: RedisPool,
}

impl RedisKvStore {
    pub fn new(pool: RedisPool) -> RedisKvStore {
        RedisKvStore { pool }
    }

    async fn get_connection(&self) -> Result<PooledConnection<'_, redis::Client>, AppError> {
        self.pool
            .get()
            .await
            .map_err(|err| errors::ErrRedisClient.with_cause(err, "get redis connection"))
    }
}

impl RedisKvStore {
    pub async fn set<T>(&self, key: &str, value: T) -> Result<(), AppError>
    where
        T: ToSingleRedisArg + Send + Sync,
    {
        let mut conn = self.get_connection().await?;
        let _: () = conn
            .set(key, value)
            .await
            .map_err(|err| errors::ErrRedisClient.with_cause(err, "set redis value"))?;
        Ok(())
    }

    pub async fn set_ex<T>(&self, key: &str, value: T, seconds: u64) -> Result<(), AppError>
    where
        T: ToSingleRedisArg + Send + Sync,
    {
        let mut conn = self.get_connection().await?;
        let _: () = conn
            .set_ex(key, value, seconds)
            .await
            .map_err(|err| errors::ErrRedisClient.with_cause(err, "set redis value with expiry"))?;
        Ok(())
    }

    /// Reads the value for a key. Missing keys and deserialization failures are errors.
    pub async fn get<T>(&self, key: &str) -> Result<T, AppError>
    where
        T: FromRedisValue,
    {
        let mut conn = self.get_connection().await?;
        conn.get::<_, T>(key)
            .await
            .map_err(|err| errors::ErrRedisClient.with_cause(err, "get redis key"))
    }

    /// Deletes a key
    pub async fn del(&self, key: &str) -> Result<(), AppError> {
        let mut conn = self.get_connection().await?;
        let _: () = conn
            .del(key)
            .await
            .map_err(|err| errors::ErrRedisClient.with_cause(err, "delete redis key"))?;
        Ok(())
    }
}
