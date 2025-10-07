use std::time::Duration;

use derivative::Derivative;
use r2d2::Pool;
use redis::Client;
use serde::Deserialize;
use tracing::info;

pub type RedisPool = Pool<Client>;
/// Redis configuration structure for connecting to Redis server
///
/// This struct holds the configuration parameters needed to establish
/// a connection to a Redis server, including database selection and
/// connection URL.
#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
pub struct RedisConf {
    /// Redis connection URL in format: redis://[username:password@]host[:port][/database]
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
    /// Initializes and returns a Redis connection pool
    ///
    /// This asynchronous function creates a Redis client using the configured URL,
    /// then builds a connection pool with the specified parameters:
    /// - Maximum pool size
    /// - Connection lifetime in seconds
    /// - Minimum number of idle connections
    ///
    /// # Returns
    /// - `Ok(RedisPool)` on successful pool creation
    /// - `Err(anyhow::Error)` if client creation or pool building fails
    ///
    /// # Errors
    /// Returns an error if:
    /// - Redis client cannot be created with the provided URL
    /// - Connection pool cannot be built with the specified parameters
    pub async fn init_pool(&self) -> anyhow::Result<RedisPool> {
        let client = Client::open(self.url.as_str())
            .map_err(|err| anyhow::anyhow!("build redis client error {}", err))?;
        let pool = r2d2::Pool::builder()
            .max_size(self.max_size)
            .max_lifetime(Some(Duration::from_secs(self.lifetime_secs)))
            .min_idle(Some(self.min_idle))
            .build(client)
            .map_err(|err| anyhow::anyhow!("build redis pool error {}", err))?;

        info!("Init redis client success");
        Ok(pool)
    }
}
