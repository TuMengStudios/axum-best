use serde::Deserialize;
use smart_default::SmartDefault;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::fmt::time::ChronoLocal;

#[derive(Debug, Deserialize, SmartDefault)]
pub struct LogConfig {
    /// Log level (debug, info, warn, error)
    #[default("info")]
    level: String,

    /// Maximum number of log files to keep
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
            other @ _ => {
                println!("not found {other} use info");
                Level::INFO
            }
        }
    }

    /// Converts the string rotation setting to Rotation enum
    fn get_rotation(&self) -> Rotation {
        match self.rotation.as_str() {
            "minutely" => Rotation::MINUTELY,
            "hourly" => Rotation::HOURLY,
            "daily" => Rotation::DAILY,
            "never" => Rotation::NEVER,
            other @ _ => {
                println!("other setting {other} use {:?}", Rotation::HOURLY);
                Rotation::HOURLY
            }
        }
    }

    /// Checks if the log format is JSON
    fn is_json(&self) -> bool {
        match self.format.to_lowercase().as_str() {
            "json" => true,
            other @ _ => {
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
    pub fn init_log(&self) -> anyhow::Result<WorkerGuard> {
        let file_appender = tracing_appender::rolling::Builder::new()
            .rotation(self.get_rotation())
            .max_log_files(self.max_files as usize)
            .filename_prefix(self.file_name.clone())
            .filename_suffix(self.suffix.clone())
            .build(self.dir.clone())
            .expect("build file appender failed");

        let (writer, guard) = tracing_appender::non_blocking(file_appender);

        let timer = ChronoLocal::new(self.time_format.clone());

        let builder = tracing_subscriber::fmt()
            .with_ansi(false)
            .with_timer(timer)
            .with_target(true)
            .with_line_number(true)
            .with_file(true)
            .with_level(true)
            .with_max_level(self.log_level())
            .with_writer(writer);

        // json format
        if self.is_json() {
            builder.json().init();
        } else {
            builder.init();
        }
        Ok(guard)
    }
}
