use anyhow::{Context, Error, Result, anyhow};
use axum::{Router, extract::Extension};
use clap::Parser;
use sqlx::sqlite::SqlitePool;
use tokio::net::TcpListener;

use crate::{args::Args, context::RequestContextExtension, database::Database, logs::Logs, server::Server};

#[derive(Clone, Default)]
pub struct ServerBuilder {
    router: Router,
}

impl ServerBuilder {
    pub fn route(
        mut self,
        path: &str,
        method_router: axum::routing::MethodRouter,
    ) -> Self {
        self.router = self.router.clone().route(path, method_router);
        self
    }

    pub async fn build(self) -> Result<Server, Error> {
        let args = Args::try_parse()?;
        let logs = Logs::register(&args).with_context(|| anyhow!("Failed to register logs."))?;
        let database = Database::register_and_migrate(&args)
            .await
            .with_context(|| anyhow!("Failed to register and migrate database."))?;
        let listener = TcpListener::bind((args.address.as_ref(), args.port))
            .await
            .with_context(|| format!("Failed to open TcpListener on \"{}:{}\".", args.address, args.port))?;
        let router = self.router.layer(Extension(database.pool.clone())).with_trace_layer();

        Ok(Server::new(router, listener, args, logs, database))
    }

    pub async fn build_and_serve(self) -> Result<(), Error> {
        let server = self.build().await.with_context(|| "Failed to build server.")?;
        server.serve().await.with_context(|| "Failed to serve server.")?;

        Ok(())
    }
}
