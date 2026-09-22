//! Fire-and-forget background task dispatcher shared by the whole application.
//!
//! Wraps [`noworkers::WorkerPool`] behind a thin handle that can live on both
//! [`AppContext`](crate::app::context::AppContext) (owner, usable across the
//! whole lifecycle) and [`AppState`](crate::core::state::AppState) (clone for
//! request-time use). Both hold the same pool: cloning only bumps an `Arc`,
//! and the capacity is global.

use std::future::Future;
use std::time::Duration;

use noworkers::WorkerPool;
use serde::Deserialize;
use tracing::Instrument;

/// Background task pool configuration.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WorkerConf {
    /// Maximum number of background tasks holding a pool slot at once.
    /// Defaults to twice the number of CPU cores.
    #[serde(default = "default_worker_capacity")]
    pub capacity: usize,

    /// How long a task submission waits for a free pool slot, in seconds.
    /// A value of 0 falls back to the pool's built-in default (1s).
    #[serde(default = "default_worker_spawn_timeout_secs")]
    pub spawn_timeout_secs: u64,
}

impl Default for WorkerConf {
    fn default() -> Self {
        Self {
            capacity: default_worker_capacity(),
            spawn_timeout_secs: default_worker_spawn_timeout_secs(),
        }
    }
}

fn default_worker_capacity() -> usize {
    std::thread::available_parallelism()
        .map(|cpus| cpus.get() * 2)
        .unwrap_or(2)
}

fn default_worker_spawn_timeout_secs() -> u64 {
    1
}

/// A cheap `Clone` handle over the shared background task pool.
///
/// Fire-and-forget semantics: awaiting [`WorkerDispatcher::spawn`] submits the
/// task — it resolves once the task is admitted to the pool (waiting up to
/// the spawn timeout for a free slot on a saturated pool), after which the
/// task runs detached on the Tokio runtime. A dropped submission is reported
/// through `tracing` by the pool. Detached tasks are not drained during
/// graceful shutdown; a task still running when the process exits is lost.
#[derive(Debug, Clone)]
pub struct WorkerDispatcher {
    pool: WorkerPool,
}

impl WorkerDispatcher {
    /// Builds a dispatcher from the `[worker]` configuration section.
    ///
    /// A zero `capacity` is clamped to one so a misconfigured pool cannot
    /// reject every submission.
    pub fn new(conf: &WorkerConf) -> WorkerDispatcher {
        WorkerDispatcher {
            pool: WorkerPool::new(
                conf.capacity.max(1),
                Duration::from_secs(conf.spawn_timeout_secs),
                "worker",
            ),
        }
    }

    /// Submits a fire-and-forget background task to the pool.
    ///
    /// Awaiting the returned future only submits the task: it resolves once
    /// the task is admitted (so a saturated pool adds at most the spawn
    /// timeout of latency before the submission is dropped), while the task
    /// itself runs detached — its completion is never awaited here.
    ///
    /// `timeout` is the execution time budget chosen by the business side for
    /// this task; `Duration::ZERO` runs it without an execution time limit.
    ///
    /// The future is instrumented with the caller's current tracing span, so
    /// its logs and OTel traces stay linked to the originating request.
    pub async fn spawn<F>(&self, timeout: Duration, future: F)
    where
        F: Future + Send + 'static,
    {
        self.pool
            .spawn_task(timeout, future.instrument(tracing::Span::current()))
            .await;
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use tokio::time::sleep;

    use super::{WorkerConf, WorkerDispatcher};

    #[tokio::test]
    async fn spawned_task_runs_to_completion() {
        let dispatcher = WorkerDispatcher::new(&WorkerConf::default());
        let (tx, rx) = tokio::sync::oneshot::channel();

        dispatcher
            .spawn(Duration::ZERO, async move {
                tx.send(42).ok();
            })
            .await;

        assert_eq!(rx.await.unwrap(), 42);
    }

    #[test]
    fn spawned_task_inherits_the_caller_span() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("build current-thread runtime");
        let dispatcher = WorkerDispatcher::new(&WorkerConf::default());
        let (tx, rx) = tokio::sync::oneshot::channel::<Option<String>>();
        let subscriber = tracing_subscriber::fmt().finish();

        let observed = tracing::subscriber::with_default(subscriber, || {
            runtime.block_on(async {
                let request = tracing::info_span!("http_request");
                let _entered = request.enter();

                dispatcher
                    .spawn(Duration::ZERO, async move {
                        let current = tracing::Span::current();
                        tx.send(current.metadata().map(|meta| meta.name().to_string()))
                            .ok();
                    })
                    .await;

                rx.await
            })
        })
        .expect("task should report the span it ran in");

        assert_eq!(observed, Some("http_request".to_string()));
    }

    #[tokio::test]
    async fn saturated_pool_drops_the_task_after_the_spawn_timeout() {
        let conf = WorkerConf {
            capacity: 1,
            spawn_timeout_secs: 1,
        };
        let dispatcher = WorkerDispatcher::new(&conf);
        let (release_tx, release_rx) = tokio::sync::oneshot::channel::<()>();

        dispatcher
            .spawn(Duration::ZERO, async move {
                release_rx.await.ok();
            })
            .await;

        // The pool is saturated: awaiting submission gives up after the spawn
        // timeout instead of waiting indefinitely.
        let started = Instant::now();
        dispatcher.spawn(Duration::ZERO, async {}).await;
        assert!(started.elapsed() >= Duration::from_millis(900));

        release_tx.send(()).ok();

        // The pool recovered: new submissions are admitted.
        let (tx, rx) = tokio::sync::oneshot::channel();
        dispatcher
            .spawn(Duration::ZERO, async move {
                tx.send(()).ok();
            })
            .await;
        tokio::time::timeout(Duration::from_secs(1), rx)
            .await
            .expect("pool should admit new tasks after saturation clears")
            .unwrap();
    }

    #[tokio::test]
    async fn task_exceeding_its_execution_budget_is_aborted() {
        let dispatcher = WorkerDispatcher::new(&WorkerConf::default());
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();

        dispatcher
            .spawn(Duration::from_millis(10), async move {
                sleep(Duration::from_millis(200)).await;
                tx.send(()).ok();
            })
            .await;

        sleep(Duration::from_millis(150)).await;
        assert!(rx.is_empty());
    }

    #[test]
    fn zero_capacity_is_clamped_to_one() {
        let conf = WorkerConf {
            capacity: 0,
            ..Default::default()
        };

        assert_eq!(WorkerDispatcher::new(&conf).pool.status().capacity, 1);
    }

    #[test]
    fn default_worker_config_uses_recommended_bounds() {
        let config = WorkerConf::default();

        let expected_capacity = std::thread::available_parallelism()
            .map(|cpus| cpus.get() * 2)
            .unwrap_or(2);
        assert_eq!(config.capacity, expected_capacity);
        assert_eq!(config.spawn_timeout_secs, 1);
    }

    #[test]
    fn worker_settings_are_configurable() {
        let config: WorkerConf = toml::from_str("capacity = 8\nspawn_timeout_secs = 2")
            .expect("worker configuration should parse");

        assert_eq!(config.capacity, 8);
        assert_eq!(config.spawn_timeout_secs, 2);
    }

    #[test]
    fn worker_config_section_is_optional() {
        let config: WorkerConf = toml::from_str("").expect("empty worker section should parse");

        assert_eq!(config, WorkerConf::default());
    }
}
