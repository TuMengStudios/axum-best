use std::time::Duration;

use bb8::Pool;
use derivative::Derivative;
use redis::Client;
#[rustfmt::skip]
use serde::{Deserialize};
use tracing::info;

/// Asynchronous Redis connection pool (bb8, the tokio version of r2d2)
pub type RedisPool = Pool<Client>;

/// Redis configuration structure for connecting to Redis server
///
/// This struct holds the configuration parameters needed to establish
/// a connection to a Redis server, including database selection and
/// connection URL.
#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
pub struct RedisConf {
    /// Redis connection URL in format: `redis://[username:password@]host[:port][/database]`
    /// Example: redis://127.0.0.1:6379/0
    #[derivative(Debug = "ignore")]
    pub url: String,

    /// Maximum lifetime of connections in the pool in seconds
    ///
    /// This determines how long a connection can remain in the pool before being closed
    /// and replaced with a new connection. Helps prevent stale connections.
    pub lifetime_secs: u64,

    /// Maximum number of connections in the pool
    ///
    /// This limits the total number of connections that can be created and maintained
    /// in the connection pool. Helps control resource usage.
    pub max_size: u32,

    /// Minimum number of idle connections to maintain in the pool
    ///
    /// This ensures that a certain number of connections are kept ready for immediate use,
    /// reducing connection establishment overhead for frequent operations.
    pub min_idle: u32,
}

impl RedisConf {
    /// Initializes and returns an async Redis connection pool (bb8)
    ///
    /// Builds a Redis client from the configured URL, then builds a tokio-based
    /// pool with the specified parameters:
    /// - Maximum pool size
    /// - Connection lifetime in seconds
    /// - Minimum number of idle connections
    ///
    /// # Returns
    /// - `Ok(RedisPool)` on successful pool creation
    /// - `Err(anyhow::Error)` if client creation or pool building fails
    pub async fn init_pool(&self) -> anyhow::Result<RedisPool> {
        let client = Client::open(self.url.as_str())
            .map_err(|err| anyhow::anyhow!("build redis client error {}", err))?;
        let pool = Pool::builder()
            .max_size(self.max_size)
            .max_lifetime(Some(Duration::from_secs(self.lifetime_secs)))
            .min_idle(Some(self.min_idle))
            .test_on_check_out(true)
            .build(client)
            .await
            .map_err(|err| anyhow::anyhow!("build redis pool error {}", err))?;

        info!("Init redis client success");
        Ok(pool)
    }
}
