use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{fmt::writer::MakeWriterExt, prelude::__tracing_subscriber_SubscriberExt};

fn level_from(value: &String) -> tracing::Level {
    match value.as_str() {
        "TRACE" => tracing::Level::TRACE,
        "DEBUG" => tracing::Level::DEBUG,
        "INFO" => tracing::Level::INFO,
        "WARN" => tracing::Level::WARN,
        _ => tracing::Level::ERROR,
    }
}

fn rotation_from(value: &String) -> Rotation {
    match value.as_str() {
        "MINUTELY" => Rotation::MINUTELY,
        "HOURLY" => Rotation::HOURLY,
        "NEVER" => Rotation::NEVER,
        _ => Rotation::DAILY,
    }
}

pub fn init(config: &crate::utils::config::TraceConfig) -> anyhow::Result<WorkerGuard> {
    let app_name = module_path!().split("::").next().unwrap().to_owned();

    // create non blocking file writer
    let (file_writer, guard) = tracing_appender::non_blocking(RollingFileAppender::new(
        rotation_from(&config.rolling_file.rotation),
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
                    .with_max_level(level_from(&config.rolling_file.with_max_level))
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
                    .with_max_level(level_from(&config.console.with_max_level))
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

// pub struct TraceSettings {
//     pub level: tracing::Level,
//     pub app_log_only: bool,
//     pub with_file: bool,
//     pub with_line_number: bool,
//     pub with_target: bool,
// }

// pub struct RollingFileSettings {
//     pub directory: String,
//     pub file_name_prefix: String,
//     pub rotation: Rotation,
// }

// pub struct Config {
//     pub rolling_file: RollingFileSettings,
//     pub file_trace: TraceSettings,
//     pub console_trace: TraceSettings,
// }

// impl Config {
//     pub fn new(directory: &str, file_name_prefix: &str) -> Self {
//         Config {
//             rolling_file: RollingFileSettings {
//                 directory: directory.to_owned(),
//                 file_name_prefix: file_name_prefix.to_owned(),
//                 rotation: Rotation::DAILY,
//             },
//             file_trace: TraceSettings {
//                 level: tracing::Level::TRACE,
//                 app_log_only: true,
//                 with_file: true,
//                 with_line_number: true,
//                 with_target: false,
//             },
//             console_trace: TraceSettings {
//                 level: tracing::Level::TRACE,
//                 app_log_only: true,
//                 with_file: false,
//                 with_line_number: false,
//                 with_target: true,
//             },
//         }
//     }
// }

// pub fn init_with_rolling_file(config: Config) -> anyhow::Result<WorkerGuard> {
//     let app_name = module_path!().split("::").next().unwrap().to_owned();

//     // create non blocking file writer
//     let (file_writer, guard) = tracing_appender::non_blocking(RollingFileAppender::new(
//         config.rolling_file.rotation,
//         config.rolling_file.directory,
//         config.rolling_file.file_name_prefix,
//     ));

//     let file_layer = {
//         let app_only = config.file_trace.app_log_only;
//         let app_name = app_name.clone();
//         tracing_subscriber::fmt::layer()
//             .with_file(config.file_trace.with_file)
//             .with_line_number(config.file_trace.with_line_number)
//             .with_target(config.file_trace.with_target)
//             .with_ansi(false)
//             .with_writer(
//                 file_writer
//                     .with_max_level(config.file_trace.level)
//                     .with_filter(move |meta| {
//                         if app_only {
//                             meta.target().starts_with(app_name.as_str())
//                         } else {
//                             true
//                         }
//                     })
//             )
//     };

//     let console_layer = {
//         let app_only = config.console_trace.app_log_only;

//         tracing_subscriber::fmt::layer()
//             .with_file(config.console_trace.with_file)
//             .with_line_number(config.console_trace.with_line_number)
//             .with_target(config.console_trace.with_target)
//             .with_ansi(true)
//             .with_writer(
//                 std::io::stdout
//                     .with_max_level(config.console_trace.level)
//                     .with_filter(move |meta| {
//                         if app_only {
//                             meta.target().starts_with(app_name.as_str())
//                         } else {
//                             true
//                         }
//                     })
//             )
//     };

//     // combine
//     let subscriber = tracing_subscriber::registry()
//         .with(file_layer)
//         .with(console_layer);

//     // make it global
//     tracing::subscriber::set_global_default(subscriber)?;

//     // hold required for nonblocking
//     Ok(guard)
// }
