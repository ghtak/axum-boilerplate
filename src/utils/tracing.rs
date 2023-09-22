use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{fmt::writer::MakeWriterExt, prelude::__tracing_subscriber_SubscriberExt};

pub struct TraceSettings {
    pub level: tracing::Level,
    pub app_log_only: bool,
    pub with_file: bool,
    pub with_line_number: bool,
    pub with_target: bool,
}

pub struct RollingFileSettings {
    pub directory: &'static str,
    pub file_name_prefix: &'static str,
    pub rotation: Rotation,
}

pub struct Config {
    pub rolling_file: RollingFileSettings,
    pub file_trace: TraceSettings,
    pub console_trace: TraceSettings,
}

impl Config {
    pub fn new(directory: &'static str, file_name_prefix: &'static str) -> Self {
        Config {
            rolling_file: RollingFileSettings {
                directory,
                file_name_prefix,
                rotation: Rotation::DAILY,
            },
            file_trace: TraceSettings {
                level: tracing::Level::TRACE,
                app_log_only: true,
                with_file: true,
                with_line_number: true,
                with_target: false,
            },
            console_trace: TraceSettings {
                level: tracing::Level::TRACE,
                app_log_only: true,
                with_file: false,
                with_line_number: false,
                with_target: true,
            },
        }
    }
}

pub fn init_with_rolling_file(config: Config) -> anyhow::Result<WorkerGuard> {
    // create non blocking file writer
    let (file_writer, guard) = tracing_appender::non_blocking(RollingFileAppender::new(
        config.rolling_file.rotation,
        config.rolling_file.directory,
        config.rolling_file.file_name_prefix,
    ));

    let file_layer = {
        let app_only = config.file_trace.app_log_only;
        let app_name = module_path!().split("::").next().unwrap().to_owned();
        tracing_subscriber::fmt::Layer::default()
            .with_file(config.file_trace.with_file)
            .with_line_number(config.file_trace.with_line_number)
            .with_target(config.file_trace.with_target)
            .with_ansi(false)
            .with_writer(
                file_writer
                    .with_max_level(config.file_trace.level)
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
        let app_only = config.console_trace.app_log_only;
        let app_name = module_path!().split("::").next().unwrap().to_owned();
        tracing_subscriber::fmt::Layer::default()
            .with_file(config.console_trace.with_file)
            .with_line_number(config.console_trace.with_line_number)
            .with_target(config.console_trace.with_target)
            .with_ansi(true)
            .with_writer(
                std::io::stdout
                    .with_max_level(config.console_trace.level)
                    .with_filter(move |meta| {
                        if app_only {
                            meta.target().starts_with(app_name.as_str())
                        } else {
                            true
                        }
                    }),
            )
    };

    // combine
    let subscriber = tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer);

    // make it global
    tracing::subscriber::set_global_default(subscriber)?;

    // hold required for nonblocking
    Ok(guard)
}
