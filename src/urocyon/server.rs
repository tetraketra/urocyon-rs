use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

use crate::urocyon::{args::Args, database::Database, logs::Logs};

#[allow(dead_code)]
pub struct Server {
    pub args: Args,
    pub database: Database,

    logs: Logs,
    router: Router,
    listener: TcpListener,
}

impl Server {
    pub(crate) fn new(
        router: Router,
        listener: TcpListener,
        args: Args,
        logs: Logs,
        database: Database,
    ) -> Self {
        Self {
            router,
            listener,
            args,
            logs,
            database,
        }
    }

    pub async fn serve(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router).await
    }
}
