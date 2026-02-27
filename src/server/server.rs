use crate::server::args::Args;
use crate::server::logs::Logs;

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

#[allow(dead_code)]
pub struct Server {
    router: Router,
    listener: TcpListener,
    args: Args,
    logs: Logs,
}

impl Server {
    pub(crate) fn new(router: Router, listener: TcpListener, args: Args, logs: Logs) -> Self {
        Self {
            router,
            listener,
            args,
            logs,
        }
    }

    pub async fn serve(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router).await
    }
}
