# task-pool

项目内部的 Tokio 任务池：为异步任务提供并发槽位限制与执行超时控制。

## 提交任务

两个提交方法都接收 `timeout: Duration`，仅控制**任务自身执行**时长，`Duration::ZERO` 表示不限：

| 方法 | 执行方式 | 返回值 |
| --- | --- | --- |
| `submit_task` | 在调用方任务里原地 `await`，等待完成 | `Result<F::Output, TaskError>` |
| `spawn_task` | 等到槽位后 `tokio::spawn` 丢后台，立即返回 | 无，错误仅写 `tracing` |

`timeout` 不约束**等待池槽位**的耗时。等待槽位超时由 `Pool::new` 的 `spawn_timeout` 控制，两个提交方法在执行任务前都会先 `await` 申请一个空闲槽位：

- `submit_task` 在拿不到槽位时返回 `Err(TaskError::SpawnTimeout)`。
- `spawn_task` 在拿不到槽位时静默丢弃任务（仅写日志），调用方无感知。
- `spawn_timeout` 传 `Duration::ZERO` 时实际使用 1s。

## 示例

```rust
use std::time::Duration;
use task_pool::Pool;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::new(16, Duration::from_millis(100), "background");

    // 同步等待结果，30s 执行超时
    let output = pool
        .submit_task(Duration::from_secs(30), async { 1 + 1 })
        .await?;
    assert_eq!(output, 2);

    // 不限时执行
    pool.submit_task(Duration::ZERO, async { 2 + 2 }).await?;

    // 后台任务，30s 执行超时，错误只记日志
    pool.spawn_task(Duration::from_secs(30), async {}).await;

    Ok(())
}
```

## 错误类型

`TaskError`：

- `SpawnTimeout`：等待池槽位超时，受 `Pool::new` 的 `spawn_timeout` 约束。
- `RunTimeout`：任务执行超时，受提交方法的 `timeout` 参数约束。
- `SpawnSemaphoreAcquireError`：信号量获取失败。仅当信号量被显式 `close()` 时返回，本 crate 不会主动关闭，正常流程不会遇到。

`submit_task` 会把以上错误返回给调用方；`spawn_task` 全部内部消化，仅写日志。

## 边界

该 crate 是进程内任务池，不提供持久化、进程崩溃恢复或跨进程共享能力。
