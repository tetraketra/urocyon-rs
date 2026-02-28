use anyhow::{Error, Result, anyhow};
use clap::error::ErrorKind;
use tracing_subscriber::{filter::LevelFilter, prelude::*};

use crate::urocyon::args::Args;

#[allow(dead_code)]
pub struct LogManager {
    guard: tracing_appender::non_blocking::WorkerGuard,
}

impl LogManager {
    pub fn register(args: &Args) -> Result<Self> {
        let path = std::path::Path::new(&args.log_path);
        let path_dir = path
            .parent()
            .ok_or_else(|| anyhow!("Logs path `{}` has no parent.", path.display()))?;
        let path_prefix = path
            .file_name()
            .ok_or_else(|| anyhow!("Log path `{}` does not end in a base file name.", path.display()))?;

        let (writer, guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(path_dir, path_prefix));

        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_writer(writer)
                    .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
                    .with_current_span(true)
                    .flatten_event(true),
            )
            .with(
                tracing_subscriber::EnvFilter::default()
                    .add_directive((LevelFilter::from_level(args.log_level.clone().into())).into()),
            )
            .init();

        Ok(LogManager { guard })
    }
}
