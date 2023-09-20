use std::io;
use std::path::Path;
use tracing::{debug, error, info, trace, warn, Level};
use tracing_appender::{non_blocking::WorkerGuard, rolling::{RollingFileAppender, Rotation}};
use tracing_subscriber::{
    fmt::{self, writer::MakeWriterExt},
    prelude::*,
};

pub fn init_with_rolling_file(
    directory: &'static str,
    file_name_prefix: &'static str
) -> WorkerGuard {
    let (non_blocking, guard) = tracing_appender::non_blocking(
        RollingFileAppender::new(
            Rotation::DAILY,
            directory, 
            file_name_prefix)
    );

    let file_log = tracing_subscriber::fmt()
        .with_writer(non_blocking.with_max_level(Level::WARN))
        .with_ansi(false);


    let console_log = fmt::Layer::new()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_ansi(true)
        .with_writer(std::io::stderr.with_min_level(
            tracing::Level::TRACE).or_else(std::io::stdout));
    
    guard

    // let (non_blocking, guard) = tracing_appender::non_blocking(
    //     RollingFileAppender::new(
    //         Rotation::DAILY,
    //         directory, 
    //         file_name_prefix)
    // );

    // let subscriber = tracing_subscriber::fmt()
    // .with_max_level(tracing::Level::DEBUG)
    // .with_writer(non_blocking).finish();

    // //let layer = tracing_subscriber::fmt::Layer::default().with_writer(non_blocking);

    // tracing::subscriber::set_global_default(
    //     subscriber
    // ).expect("Unable to set global tracing subscriber");
    // debug!("Tracing initialized.");
    // guard

    // let (non_blocking, guard) = tracing_appender::non_blocking(
    //     RollingFileAppender::new(
    //         Rotation::DAILY,
    //         directory, 
    //         file_name_prefix)
    // );
    // let layer = tracing_subscriber::fmt::Layer::default().with_writer(non_blocking);

    // tracing::subscriber::set_global_default(
    //     tracing_subscriber::fmt::Subscriber::builder()
    //         .with_max_level(tracing::Level::DEBUG)
    //         .with_file(true)
    //         .with_line_number(true)
    //         .with_target(false)
    //         .finish()
    //         .with(layer)
    // ).expect("Unable to set global tracing subscriber");
    // debug!("Tracing initialized.");
    // guard
}