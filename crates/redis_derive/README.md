# redis_derive

Provides separate derives for JSON Redis serialization and deserialization.
`RedisArgs` implements `redis::ToRedisArgs`, while `RedisValue` implements
`redis::FromRedisValue`.

```rust
use redis_derive::{RedisArgs, RedisValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, RedisArgs, RedisValue)]
struct Student {
    id: u64,
    name: String,
    age: u8,
    grade: String,
    scores: Vec<u32>,
}
```

The consuming crate must depend directly on `redis`, `serde` with its `derive`
feature, and `serde_json`. The derives use these standard crate names.
Serialization failures panic with `serialize value for Redis`; invalid JSON
returns a Redis `TypeError` with the message `JSON parse error`.
