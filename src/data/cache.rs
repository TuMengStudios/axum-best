use std::time::Duration;

use bb8::{ManageConnection, Pool};
use derivative::Derivative;
use redis::{Client, ErrorKind, IntoConnectionInfo, RedisError};
#[rustfmt::skip]
use serde::{Deserialize};
use tracing::info;

/// Asynchronous Redis connection pool (bb8, the tokio version of r2d2)
pub type RedisPool = Pool<RedisConnectionManager>;

/// Redis connection manager for the bb8 connection pool.
#[derive(Clone, Debug)]
pub struct RedisConnectionManager {
    client: Client,
}

impl RedisConnectionManager {
    /// Creates a manager from a Redis connection URL.
    pub fn new<T: IntoConnectionInfo>(info: T) -> Result<Self, RedisError> {
        Ok(Self {
            client: Client::open(info.into_connection_info()?)?,
        })
    }
}

impl ManageConnection for RedisConnectionManager {
    type Connection = redis::aio::MultiplexedConnection;
    type Error = RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_multiplexed_async_connection().await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(conn).await?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((ErrorKind::Extension, "ping request").into()),
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

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
    /// Builds a `RedisConnectionManager` from the configured URL, then builds
    /// a tokio-based pool with the specified parameters:
    /// - Maximum pool size
    /// - Connection lifetime in seconds
    /// - Minimum number of idle connections
    ///
    /// # Returns
    /// - `Ok(RedisPool)` on successful pool creation
    /// - `Err(anyhow::Error)` if manager creation or pool building fails
    pub async fn init_pool(&self) -> anyhow::Result<RedisPool> {
        let manager = RedisConnectionManager::new(self.url.as_str())
            .map_err(|err| anyhow::anyhow!("build redis client error {}", err))?;
        let pool = Pool::builder()
            .max_size(self.max_size)
            .max_lifetime(Some(Duration::from_secs(self.lifetime_secs)))
            .min_idle(Some(self.min_idle))
            .build(manager)
            .await
            .map_err(|err| anyhow::anyhow!("build redis pool error {}", err))?;

        info!("Init redis client success");
        Ok(pool)
    }
}
