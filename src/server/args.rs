use clap::{Parser, ValueEnum};
use serde::Serialize;
use tracing::Level;

#[derive(Debug, Clone, ValueEnum, Serialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

#[derive(Parser, Debug, Serialize)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "0.0.0.0")]
    pub address: String,
    #[arg(short, long, default_value_t = 7654)]
    pub port: u32,
    #[arg(long, default_value = "urocyon.database.sqlite3")]
    pub db_path: String,
    #[arg(long, default_value = "urocyon.log.ndjson")]
    pub log_path: String,
    #[arg(long, default_value = "info")]
    pub log_level: LogLevel,
}

impl Args {
    pub fn parse_or_exit() -> Self {
        Self::try_parse().unwrap_or_else(|e| {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        })
    }
}
