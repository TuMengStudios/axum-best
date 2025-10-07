use derivative::Derivative;
use r2d2::PooledConnection;
use redis::Client;
use serde::Deserialize;
use sqlx::MySqlPool;
use tracing::error;

use crate::core::rest::AppError;
use crate::data::cache::RedisPool;
use crate::errors;

#[derive(Deserialize, Derivative, Clone)]
#[derivative(Debug)]
pub struct WeChatConf {
    pub appid: String,
    #[derivative(Debug = "ignore")]
    pub secret: String,
}

#[allow(unused)]
#[derive(Clone)]
pub struct AppState {
    pub db_conn: MySqlPool,
    pub redis_pool: RedisPool,
    pub wechat: WeChatConf,
}

impl AppState {
    pub fn new(conn: MySqlPool, redis_pool: RedisPool, wechat: WeChatConf) -> AppState {
        AppState {
            db_conn: conn,
            redis_pool,
            wechat,
        }
    }

    /// Returns a cloned database connection pool
    ///
    /// This method provides access to the application's database connection pool
    /// by returning a cloned instance. The clone operation is lightweight as
    /// `MySqlPool` uses Arc internally for shared ownership.
    ///
    /// # Returns
    /// - `MySqlPool`: A cloned instance of the database connection pool
    ///
    /// # Note
    /// - The returned pool can be used to execute database queries
    /// - Each clone shares the same underlying connection pool
    /// - This method does not establish new connections, it reuses existing ones
    pub fn get_conn(&self) -> MySqlPool {
        self.db_conn.clone()
    }

    pub fn get_redis_client(&self) -> core::result::Result<PooledConnection<Client>, AppError> {
        let conn = self.redis_pool.get().map_err(|err| {
            error!("get redis client error {}", err);
            errors::ErrRedisClient.clone()
        })?;
        Ok(conn)
    }
}
