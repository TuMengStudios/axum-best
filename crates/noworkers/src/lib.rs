#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use thiserror::Error;
use tokio::sync::Semaphore;
use tracing::error;

/// Errors returned while admitting or running a task.
#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum TaskError {
    #[error("task spawn timeout")]
    SpawnTimeout,
    #[error("task run timeout")]
    RunTimeout,
    #[error("task spawn semaphore acquire error")]
    SpawnSemaphoreAcquireError,
}

impl From<tokio::sync::AcquireError> for TaskError {
    fn from(_: tokio::sync::AcquireError) -> Self {
        Self::SpawnSemaphoreAcquireError
    }
}

/// A bounded pool for Tokio tasks.
#[derive(Debug, Clone)]
pub struct WorkerPool {
    name: Arc<String>,
    spawn_timeout: Duration,
    limiter: Arc<Semaphore>,
    capacity: usize,
}

/// A point-in-time snapshot of a pool's work status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkStatus {
    /// WorkerPool name.
    pub name: String,
    /// Total number of concurrent slots.
    pub capacity: usize,
    /// Number of free slots.
    pub available_permits: usize,
    /// Number of slots currently held by tasks.
    pub busy_permits: usize,
}

impl WorkerPool {
    /// Creates a pool with `capacity` concurrent slots.
    ///
    /// `spawn_timeout` bounds how long a submission waits for a free slot;
    /// `Duration::ZERO` defaults to 1s. `name` is used in log fields; an
    /// empty name falls back to `worker`.
    pub fn new(capacity: usize, spawn_timeout: Duration, name: &str) -> Self {
        let name = if name.is_empty() { "worker" } else { name }.to_string();
        let spawn_timeout = if spawn_timeout.is_zero() {
            Duration::from_secs(1)
        } else {
            spawn_timeout
        };

        Self {
            name: Arc::new(name),
            spawn_timeout,
            limiter: Arc::new(Semaphore::new(capacity)),
            capacity,
        }
    }

    /// Returns a snapshot of the pool work status.
    pub fn status(&self) -> WorkStatus {
        let available_permits = self.limiter.available_permits();

        WorkStatus {
            name: self.name.as_str().to_string(),
            capacity: self.capacity,
            available_permits,
            busy_permits: self.capacity - available_permits,
        }
    }

    /// Submits a task to the pool and waits for it to finish, returning the
    /// task's output.
    ///
    /// The task runs in the caller's context without an extra `tokio::spawn`;
    /// the pool only bounds how many tasks may hold a slot at once. A
    /// non-zero `timeout` aborts the task with [`TaskError::RunTimeout`] if
    /// it does not finish in time; `Duration::ZERO` runs it without an
    /// execution timeout.
    pub async fn submit_task<F>(
        &self,
        timeout: Duration,
        future: F,
    ) -> std::result::Result<F::Output, TaskError>
    where
        F: Future + Send + 'static,
    {
        let _permit = self.acquire_sem().await?;

        if timeout.is_zero() {
            Ok(future.await)
        } else {
            tokio::time::timeout(timeout, future)
                .await
                .map_err(|_| TaskError::RunTimeout)
        }
    }

    /// Submits a fire-and-forget task and returns as soon as it is admitted.
    ///
    /// Unlike [`WorkerPool::submit_task`], the task runs detached on the Tokio
    /// runtime: the caller does not wait for completion and gets no result
    /// back. The future may produce any output; it is awaited and discarded.
    /// All errors are swallowed and only reported through `tracing`.
    pub async fn spawn_task<F>(&self, timeout: Duration, future: F)
    where
        F: Future + Send + 'static,
    {
        let pool_name = Arc::clone(&self.name);

        let permit = match self.acquire_sem().await {
            Ok(permit) => permit,
            Err(_) => return,
        };

        tokio::spawn(async move {
            let _permit = permit;

            if timeout.is_zero() {
                let _ = future.await;
            } else if tokio::time::timeout(timeout, future).await.is_err() {
                error!(pool = pool_name.as_str(), error = ?TaskError::RunTimeout, "task run timeout");
            }
        });
    }

    /// Acquires one pool slot, waiting up to `spawn_timeout`.
    ///
    /// Returns an owned permit on success; the slot is released when the
    /// permit is dropped. If the wait exceeds `spawn_timeout`, the elapsed
    /// error is logged and [`TaskError::SpawnTimeout`] is returned. A closed
    /// semaphore surfaces as [`TaskError::SpawnSemaphoreAcquireError`].
    async fn acquire_sem(
        &self,
    ) -> std::result::Result<tokio::sync::OwnedSemaphorePermit, TaskError> {
        let acquired =
            tokio::time::timeout(self.spawn_timeout, self.limiter.clone().acquire_owned()).await;

        match acquired {
            Ok(permit) => Ok(permit?),
            Err(error) => {
                error!(pool = self.name.as_str(), timeout = ?self.spawn_timeout, error = ?error, "task failed to acquire pool slot");
                Err(TaskError::SpawnTimeout)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use tokio::time::sleep;

    use super::{TaskError, WorkStatus, WorkerPool};

    #[test]
    fn falls_back_to_default_name() {
        assert_eq!(WorkerPool::new(1, Duration::ZERO, "").status().name, "worker");
        assert_eq!(WorkerPool::new(1, Duration::ZERO, "bg").status().name, "bg");
    }

    #[test]
    fn reports_pool_status() {
        let status = WorkerPool::new(2, Duration::ZERO, "test").status();

        assert_eq!(
            status,
            WorkStatus {
                name: "test".to_string(),
                capacity: 2,
                available_permits: 2,
                busy_permits: 0,
            }
        );
    }

    #[tokio::test]
    async fn runs_bounded_tasks() {
        let pool = WorkerPool::new(2, Duration::ZERO, "test");
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..2 {
            let counter = Arc::clone(&counter);
            pool.submit_task(Duration::ZERO, async move {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .await
            .unwrap();
        }

        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn reports_spawn_timeout() {
        let pool = WorkerPool::new(1, Duration::from_millis(10), "test");
        let pool_clone = pool.clone();
        let first = tokio::spawn(async move {
            pool_clone
                .submit_task(Duration::ZERO, async {
                    sleep(Duration::from_millis(50)).await;
                })
                .await
        });

        sleep(Duration::from_millis(5)).await;
        assert_eq!(
            pool.submit_task(Duration::ZERO, async {})
                .await
                .unwrap_err(),
            TaskError::SpawnTimeout
        );
        first.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn reports_run_timeout() {
        let pool = WorkerPool::new(1, Duration::ZERO, "test");
        let result = pool
            .submit_task(Duration::from_millis(10), async {
                sleep(Duration::from_millis(50)).await;
            })
            .await;

        assert_eq!(result, Err(TaskError::RunTimeout));
    }

    #[tokio::test]
    async fn returns_task_output() {
        let pool = WorkerPool::new(2, Duration::ZERO, "test");
        let output = pool
            .submit_task(Duration::ZERO, async { 1 + 1 })
            .await
            .unwrap();

        assert_eq!(output, 2);
    }

    #[tokio::test]
    async fn spawns_detached_tasks() {
        let pool = WorkerPool::new(2, Duration::ZERO, "test");
        let counter = Arc::new(AtomicUsize::new(0));
        let task_counter = Arc::clone(&counter);

        pool.spawn_task(Duration::ZERO, async move {
            task_counter.fetch_add(1, Ordering::SeqCst);
        })
        .await;

        sleep(Duration::from_millis(20)).await;
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn spawn_task_discards_output() {
        let pool = WorkerPool::new(2, Duration::ZERO, "test");

        pool.spawn_task(Duration::ZERO, async { 1 + 1 }).await;

        sleep(Duration::from_millis(20)).await;
    }

    #[tokio::test]
    async fn spawn_task_swallows_spawn_timeout() {
        let pool = WorkerPool::new(1, Duration::from_millis(10), "test");
        let pool_clone = pool.clone();
        let first = tokio::spawn(async move {
            pool_clone
                .spawn_task(Duration::ZERO, async {
                    sleep(Duration::from_millis(50)).await;
                })
                .await
        });

        sleep(Duration::from_millis(5)).await;
        pool.spawn_task(Duration::ZERO, async {}).await;
        first.await.unwrap();
    }

    #[tokio::test]
    async fn spawn_task_aborts_on_run_timeout() {
        let pool = WorkerPool::new(2, Duration::ZERO, "test");
        let counter = Arc::new(AtomicUsize::new(0));
        let task_counter = Arc::clone(&counter);

        pool.spawn_task(Duration::from_millis(10), async move {
            sleep(Duration::from_millis(50)).await;
            task_counter.fetch_add(1, Ordering::SeqCst);
        })
        .await;

        sleep(Duration::from_millis(100)).await;
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }
}
