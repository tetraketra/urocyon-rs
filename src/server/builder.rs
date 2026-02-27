use crate::server::args::Args;
use crate::server::logs::Logs;
use crate::server::server::Server;
use crate::server::traceext::TraceExtension;

use anyhow::{Error, Result};
use axum::{Router, extract::Extension};
use sqlx::sqlite::SqlitePool;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct ServerBuilder {
    router: Router,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn route(mut self, path: &str, method_router: axum::routing::MethodRouter) -> Self {
        self.router = self.router.clone().route(path, method_router);
        self
    }

    pub async fn build(self) -> Result<Server, Error> {
        let args = Args::parse_or_exit();
        let logs = Logs::register(&args)?;
        let pool = SqlitePool::connect(&format!("sqlite://{}?mode=rwc", args.db_path)).await?;
        let listener = TcpListener::bind(format!("{}:{}", args.address, args.port)).await?;
        let router = self.router.layer(Extension(pool)).with_trace_layer();

        Ok(Server::new(router, listener, args, logs))
    }

    pub async fn build_and_serve(self) -> Result<(), Error> {
        let server = self.build().await?;
        let _ = server.serve().await?;

        Ok(())
    }
}
