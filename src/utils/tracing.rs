use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

pub fn init_with_rolling_file(
    directory: &'static str,
    file_name_prefix: &'static str,
) -> anyhow::Result<WorkerGuard> {
    let file_log = tracing_appender::rolling::daily(directory, file_name_prefix);

    let (non_blocking, guard) = tracing_appender::non_blocking(file_log);

    let layer = tracing_subscriber::fmt::Layer::default()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_ansi(false)
        .with_writer(non_blocking);

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_ansi(true)
        .finish()
        .with(layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(guard)

    // Log all events to a rolling log file.
    // let logfile = tracing_appender::rolling::daily(directory, file_name_prefix);
    // // Log `INFO` and above to stdout.
    // let stdout = std::io::stdout.with_max_level(tracing::Level::DEBUG);
    // let subscriber = tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::TRACE)
    //     .with_file(true)
    //     .with_line_number(true)
    //     .with_target(false)
    //     .with_ansi(true)
    //     .with_writer(stdout.and(logfile))
    //     .finish();

    // tracing::subscriber::set_global_default(
    //     subscriber
    // ).expect("Unable to set global tracing subscriber");
    // trace!("Tracing initialized.");

    //guard
}
