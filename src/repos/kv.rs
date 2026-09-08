use async_trait::async_trait;

use crate::core::rest::AppError;

/// KV 存储接口：抽象 Redis 等缓存存储，具体实现由 data 层提供
#[async_trait]
pub trait KvStore: Send + Sync {
    /// 写入 key/value
    async fn set(&self, key: &str, value: &str) -> Result<(), AppError>;

    /// 读取 key 对应的值，key 不存在时返回 None
    async fn get(&self, key: &str) -> Result<Option<String>, AppError>;

    /// 删除 key
    async fn del(&self, key: &str) -> Result<(), AppError>;
}
