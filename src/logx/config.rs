use serde::Deserialize;
use smart_default::SmartDefault;
use tracing::Event;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::format::{Format, FormatEvent, FormatFields, Json, Writer};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;

const GIT_COMMIT_ID: &str = gitver::git_version!(fallback = "");

struct CommitJson<T> {
    inner: Format<Json, T>,
}

impl<S, N, T> FormatEvent<S, N> for CommitJson<T>
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
    T: tracing_subscriber::fmt::time::FormatTime,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        if GIT_COMMIT_ID.is_empty() {
            return self.inner.format_event(ctx, writer, event);
        }

        let mut output = String::new();
        self.inner
            .format_event(ctx, Writer::new(&mut output), event)?;
        let mut json: serde_json::Value =
            serde_json::from_str(&output).map_err(|_| std::fmt::Error)?;
        if let serde_json::Value::Object(fields) = &mut json {
            fields.insert("commit".to_owned(), GIT_COMMIT_ID.into());
        }
        writeln!(writer, "{}", serde_json::to_string(&json).map_err(|_| std::fmt::Error)?)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, SmartDefault)]
pub struct LogConfig {
    /// Log level (debug, info, warn, error)
    #[default("info")]
    level: String,

    /// Maximum number of log files to keep (falls back to 10 if not positive)
    #[default(10)]
    max_files: i32,

    /// Log rotation strategy (minutely, hourly, daily, never)
    #[default("hourly")]
    rotation: String,

    /// Directory where log files are stored
    #[default("logs")]
    dir: String,

    /// Log file name prefix
    #[default("axum_best")]
    file_name: String,

    /// Log file suffix/extension
    #[default("log")]
    suffix: String,

    /// Time format for log timestamps
    #[default("%Y-%m-%d %H:%M:%S%.6f")]
    time_format: String,

    /// Log format ("json" or "plain")
    #[default("json")]
    format: String,
}

impl LogConfig {
    /// Converts the string log level to tracing::Level enum
    fn log_level(&self) -> Level {
        match self.level.to_lowercase().as_str() {
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            other => {
                println!("not found {other} use info");
                Level::INFO
            }
        }
    }

    /// Returns the maximum number of log files, falling back to 10 if not positive
    fn get_max_files(&self) -> usize {
        if self.max_files <= 0 {
            println!("max_files {} is invalid, use 10", self.max_files);
            10
        } else {
            self.max_files as usize
        }
    }

    /// Converts the string rotation setting to Rotation enum
    fn get_rotation(&self) -> Rotation {
        match self.rotation.as_str() {
            "minutely" => Rotation::MINUTELY,
            "hourly" => Rotation::HOURLY,
            "daily" => Rotation::DAILY,
            "never" => Rotation::NEVER,
            other => {
                println!("other setting {other} use {:?}", Rotation::HOURLY);
                Rotation::HOURLY
            }
        }
    }

    /// Checks if the log format is JSON
    fn is_json(&self) -> bool {
        match self.format.to_lowercase().as_str() {
            "json" => true,
            other => {
                println!("format {other} use plain");
                false
            }
        }
    }

    /// Initializes the logging system with the current configuration
    ///
    /// # Returns
    /// - `WorkerGuard` that must be kept alive for the duration of the program to ensure all logs
    ///   are flushed properly
    ///
    /// # Errors
    /// - Returns an error if the logging system fails to initialize
    pub fn init_log(
        &self,
        tracer: opentelemetry_sdk::trace::Tracer,
    ) -> anyhow::Result<WorkerGuard> {
        let file_appender = tracing_appender::rolling::Builder::new()
            .rotation(self.get_rotation())
            .max_log_files(self.get_max_files())
            .filename_prefix(self.file_name.clone())
            .filename_suffix(self.suffix.clone())
            .build(self.dir.clone())
            .map_err(|err| anyhow::anyhow!("build file appender failed: {err}"))?;

        let (writer, guard) = tracing_appender::non_blocking(file_appender);

        let timer = ChronoLocal::new(self.time_format.clone());

        // json format
        if self.is_json() {
            let format = tracing_subscriber::fmt::format()
                .json()
                .with_ansi(false)
                .with_timer(timer)
                .with_target(true)
                .with_line_number(true)
                .with_file(true)
                .with_level(true);
            tracing_subscriber::fmt()
                .json()
                .event_format(CommitJson { inner: format })
                .with_max_level(self.log_level())
                .with_writer(writer)
                .finish()
                .with(tracing_opentelemetry::layer().with_tracer(tracer))
                .try_init()
                .map_err(|err| anyhow::anyhow!("initialize tracing subscriber failed: {err}"))?;
        } else {
            tracing_subscriber::fmt()
                .with_ansi(false)
                .with_timer(timer)
                .with_target(true)
                .with_line_number(true)
                .with_file(true)
                .with_level(true)
                .with_max_level(self.log_level())
                .with_writer(writer)
                .finish()
                .with(tracing_opentelemetry::layer().with_tracer(tracer))
                .try_init()
                .map_err(|err| anyhow::anyhow!("initialize tracing subscriber failed: {err}"))?;
        }
        Ok(guard)
    }
}
