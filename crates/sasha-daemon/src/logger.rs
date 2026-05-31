use std::io::{stdout};
use std::fs;

use anyhow::Context;

use tracing_subscriber::filter::{EnvFilter};
use tracing_subscriber::registry::{Registry};
use tracing_subscriber::fmt::{self, format, time};
use tracing_subscriber::prelude::*;

use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_appender::non_blocking::WorkerGuard;

pub fn init_logger() -> anyhow::Result<WorkerGuard> {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let log_dir = home_dir
        .join(".local")
        .join("state")
        .join("sasha");

    fs::create_dir_all(&log_dir)
        .with_context(|| format!("Failed to create log directory: {}", log_dir.display()))?;

    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "sasha.log");
    let (non_blocking_writer, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_writer)
        .with_level(true)
        .with_target(true);

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt = format().with_timer(time::Uptime::default());
    let journal_layer = fmt::layer()
        .event_format(fmt)
        .with_ansi(true)
        .with_writer(stdout)
        .with_level(true)
        .with_target(true);
        

    let subscriber = Registry::default()
        .with(filter)
        .with(file_layer)
        .with(journal_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();

    Ok(guard)
}
