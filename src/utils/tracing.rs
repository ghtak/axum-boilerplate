use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt::writer::MakeWriterExt, prelude::__tracing_subscriber_SubscriberExt};

pub fn init_with_rolling_file(
    directory: &'static str,
    file_name_prefix: &'static str,
) -> anyhow::Result<WorkerGuard> {
    let (non_blocking, guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(
        directory,
        file_name_prefix,
    ));

    //let filter = filter::Targets::new().with_target(target, Level::TRACE);
    let app_name = module_path!().split("::").next().unwrap();

    let non_blocking = non_blocking
        .with_max_level(tracing::Level::TRACE)
        .with_filter(move |meta| return meta.target().starts_with(app_name));

    let layer = tracing_subscriber::fmt::Layer::default()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_ansi(false)
        .with_writer(non_blocking);

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_ansi(true)
        .finish()
        .with(layer);

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(guard)
}
