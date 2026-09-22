# noworkers

An internal Tokio task pool that limits concurrent asynchronous tasks and
supports task execution timeouts.

## Submitting Tasks

Both submission methods accept a `timeout: Duration`. It only limits the
task's execution time; `Duration::ZERO` disables the execution timeout:

| Method | Execution | Return value |
| --- | --- | --- |
| `submit_task` | Awaits the task in the caller's task until it completes | `Result<F::Output, TaskError>` |
| `spawn_task` | Calls `tokio::spawn` after acquiring a slot and returns immediately | None; errors are reported through `tracing` |

`timeout` does not limit the time spent waiting for a pool slot. Slot wait time
is controlled by `WorkerPool::new`'s `spawn_timeout`. Both methods acquire a free slot
before running the task:

- `submit_task` returns `Err(TaskError::SpawnTimeout)` if no slot is acquired.
- `spawn_task` silently drops the task when no slot is acquired and only logs the failure.
- `Duration::ZERO` for `spawn_timeout` defaults to one second.

## Example

```rust
use std::time::Duration;
use noworkers::WorkerPool;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let pool = WorkerPool::new(16, Duration::from_millis(100), "background");

    // Wait for the result with a 30-second execution timeout.
    let output = pool
        .submit_task(Duration::from_secs(30), async { 1 + 1 })
        .await?;
    assert_eq!(output, 2);

    // Run without an execution timeout.
    pool.submit_task(Duration::ZERO, async { 2 + 2 }).await?;

    // Run in the background with a 30-second execution timeout.
    pool.spawn_task(Duration::from_secs(30), async {}).await;

    Ok(())
}
```

## Errors

`TaskError` has the following variants:

- `SpawnTimeout`: Waiting for a pool slot exceeded the `spawn_timeout` configured by `WorkerPool::new`.
- `RunTimeout`: Task execution exceeded the `timeout` passed to the submission method.
- `SpawnSemaphoreAcquireError`: Acquiring a semaphore permit failed. This is only returned when the semaphore is explicitly closed; this crate never closes it during normal operation.

`submit_task` returns these errors to the caller. `spawn_task` handles them
internally and only reports them through `tracing`.

## Scope

This crate provides an in-process task pool. It does not provide persistence,
crash recovery, or cross-process coordination.
