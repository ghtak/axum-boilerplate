use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{fmt::writer::MakeWriterExt, prelude::__tracing_subscriber_SubscriberExt};

trait ConfigStringToValue {
    fn into_rotation(&self) -> Rotation;
    fn into_level(&self) -> tracing::Level;
}

impl ConfigStringToValue for String {
    fn into_rotation(&self) -> Rotation {
        match self.as_str() {
            "MINUTELY" => Rotation::MINUTELY,
            "HOURLY" => Rotation::HOURLY,
            "NEVER" => Rotation::NEVER,
            _ => Rotation::DAILY,
        }
    }

    fn into_level(&self) -> tracing::Level {
        match self.as_str() {
            "TRACE" => tracing::Level::TRACE,
            "DEBUG" => tracing::Level::DEBUG,
            "INFO" => tracing::Level::INFO,
            "WARN" => tracing::Level::WARN,
            _ => tracing::Level::ERROR,
        }
    }
}

pub(crate) fn init(config: &crate::utils::config::TracingConfig) -> anyhow::Result<WorkerGuard> {
    let app_name = module_path!().split("::").next().unwrap().to_owned();

    // create non blocking file writer
    let (file_writer, guard) = tracing_appender::non_blocking(RollingFileAppender::new(
        config.rolling_file.rotation.into_rotation(),
        config.rolling_file.directory.as_str(),
        config.rolling_file.file_name_prefix.as_str(),
    ));

    let file_layer = {
        let app_only = config.rolling_file.app_only;
        let app_name = app_name.clone();
        tracing_subscriber::fmt::layer()
            .with_file(config.rolling_file.with_file)
            .with_line_number(config.rolling_file.with_line_number)
            .with_target(config.rolling_file.with_target)
            .with_ansi(false)
            .with_writer(
                file_writer
                    .with_max_level(config.rolling_file.with_max_level.into_level())
                    .with_filter(move |meta| {
                        if app_only {
                            meta.target().starts_with(app_name.as_str())
                        } else {
                            true
                        }
                    }),
            )
    };

    let console_layer = {
        let app_only = config.console.app_only;
        tracing_subscriber::fmt::layer()
            .with_file(config.console.with_file)
            .with_line_number(config.console.with_line_number)
            .with_target(config.console.with_target)
            .with_ansi(true)
            .with_writer(
                std::io::stdout
                    .with_max_level(config.console.with_max_level.into_level())
                    .with_filter(move |meta| {
                        if app_only {
                            meta.target().starts_with(app_name.as_str())
                        } else {
                            true
                        }
                    }),
            )
    };

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(file_layer)
            .with(console_layer),
    )?;

    // hold required for nonblocking
    Ok(guard)
}
