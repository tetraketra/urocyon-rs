use crate::server::args::Args;

use anyhow::{Error, Result};
use tracing_subscriber::{filter::LevelFilter, prelude::*};

pub struct Logs {}
impl Logs {
    pub fn register(args: &Args) -> Result<Self, Error> {
        let (writer, _guard) = tracing_appender::non_blocking(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&args.log_path)?,
        );

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

        Ok(Logs {})
    }
}
