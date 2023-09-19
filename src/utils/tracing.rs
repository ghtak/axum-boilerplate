use tracing::debug;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

pub fn init_with_rolling_file(
    directory: String,
    file_name_prefix: String
) -> WorkerGuard {
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        directory, 
        file_name_prefix);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt::Subscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .finish()
            .with(tracing_subscriber::fmt::Layer::default().with_writer(non_blocking))
    ).expect("Unable to set global tracing subscriber");
    debug!("Tracing initialized.");
    guard
}