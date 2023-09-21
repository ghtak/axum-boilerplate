use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt::writer::MakeWriterExt, prelude::__tracing_subscriber_SubscriberExt};

pub struct RollingFileConfig {
    pub directory: &'static str,
    pub file_name_prefix: &'static str,
    pub level: tracing::Level,
    pub app_log_only: bool,
    pub with_file: bool,
    pub with_line_number: bool,
    pub with_target: bool,
}

impl RollingFileConfig {
    pub fn new(directory: &'static str, file_name_prefix: &'static str) -> Self {
        RollingFileConfig {
            directory,
            file_name_prefix,
            level: tracing::Level::TRACE,
            app_log_only: true,
            with_file: true,
            with_line_number: true,
            with_target: false,
        }
    }
}

pub struct ConsoleConfig {
    pub level: tracing::Level,
    pub app_log_only: bool,
    pub with_file: bool,
    pub with_line_number: bool,
    pub with_target: bool,
}

impl ConsoleConfig {
    pub fn default() -> Self {
        ConsoleConfig {
            level: tracing::Level::TRACE,
            app_log_only: true,
            with_file: false,
            with_line_number: false,
            with_target: true,
        }
    }
}

pub struct Config {
    pub file: RollingFileConfig,
    pub console: ConsoleConfig,
}

impl Config {
    pub fn new(directory: &'static str, file_name_prefix: &'static str) -> Self {
        Config {
            file: RollingFileConfig::new(directory, file_name_prefix),
            console: ConsoleConfig::default(),
        }
    }
}

pub fn init_with_rolling_file(config: Config) -> anyhow::Result<WorkerGuard> {
    let app_name = module_path!().split("::").next().unwrap();

    // create non blocking file writer
    let (non_blocking, guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(
        config.file.directory,
        config.file.file_name_prefix,
    ));

    // create filtered file writer
    let file_app_log_only = config.file.app_log_only;
    let non_blocking = non_blocking
        .with_max_level(config.file.level)
        .with_filter(move |meta| {
            if file_app_log_only {
                meta.target().starts_with(app_name)
            } else {
                true
            }
        });

    // create file layer
    let flle_layer = tracing_subscriber::fmt::Layer::default()
        .with_file(config.file.with_file)
        .with_line_number(config.file.with_line_number)
        .with_target(config.file.with_target)
        .with_ansi(false)
        .with_writer(non_blocking);

    // create filtered console writer
    let console_app_log_only = config.console.app_log_only;
    let console_writer = std::io::stdout
        .with_max_level(config.console.level)
        .with_filter(move |meta| {
            if console_app_log_only {
                meta.target().starts_with(app_name)
            } else {
                true
            }
        });

    // create console layer
    let console_layer = tracing_subscriber::fmt::Layer::default()
        .with_file(config.console.with_file)
        .with_line_number(config.console.with_line_number)
        .with_target(config.console.with_target)
        .with_ansi(true)
        .with_writer(console_writer);

    // combine
    let subscriber = tracing_subscriber::registry()
        .with(flle_layer)
        .with(console_layer);

    // make it global
    tracing::subscriber::set_global_default(subscriber)?;

    // hold required for nonblocking
    Ok(guard)
}
