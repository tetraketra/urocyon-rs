use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

use crate::urocyon::{args::Args, database::Database, logs::LogManager};

#[allow(dead_code)]
pub struct Server {
    pub args: Args,
    pub database: Database,

    logs: LogManager,
    router: Router,
    listener: TcpListener,
}

impl Server {
    pub(crate) fn new(
        router: Router,
        listener: TcpListener,
        args: Args,
        logs: LogManager,
        database: Database,
    ) -> Self {
        Self {
            router: router,
            listener: listener,
            args: args,
            logs,
            database,
        }
    }

    pub async fn serve(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router).await
    }
}
