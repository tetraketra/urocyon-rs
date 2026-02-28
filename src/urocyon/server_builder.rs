use anyhow::{Context, Error, Result, anyhow};
use axum::{Router, extract::Extension};
use sqlx::sqlite::SqlitePool;
use tokio::net::TcpListener;

use crate::urocyon::{args::Args, context::RequestContextExtension, database::Database, logs::Logs, server::Server};

#[derive(Clone)]
pub struct ServerBuilder {
    router: Router,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self { router: Router::new() }
    }

    pub fn route(
        mut self,
        path: &str,
        method_router: axum::routing::MethodRouter,
    ) -> Self {
        self.router = self.router.clone().route(path, method_router);
        self
    }

    pub async fn build(self) -> Result<Server, Error> {
        let args = Args::parse_or_exit_with_clap_error();
        let logs = Logs::register(&args).with_context(|| anyhow!("Failed to register logs."))?;
        let database = Database::register_and_migrate(&args)
            .await
            .with_context(|| anyhow!("Failed to register and migrate database."))?;
        let listener_addr = format!("{}:{}", args.address, args.port);
        let listener = TcpListener::bind(listener_addr.clone())
            .await
            .with_context(|| format!("Failed to open TcpListener at address `{}`.", listener_addr))?;

        let router = self.router.layer(Extension(database.pool.clone())).with_trace_layer();

        Ok(Server::new(router, listener, args, logs, database))
    }

    pub async fn build_and_serve(self) -> Result<(), Error> {
        let server = self.build().await.with_context(|| "Failed to build server.")?;
        let _ = server.serve().await.with_context(|| "Failed to serve server.")?;

        Ok(())
    }
}
