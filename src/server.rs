mod args;
use args::Args;
mod logs;
use logs::Logs;

use anyhow::{Error, Result};
use axum::{
    Router,
    body::Body,
    http::{HeaderName, Request},
};
use sqlx::sqlite::SqlitePool;
use tokio::net::TcpListener;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::Level;

pub struct Server {
    router: Router,
    listener: TcpListener,
    #[allow(dead_code)]
    args: Args,
    #[allow(dead_code)]
    logs: Logs,
}

impl Server {
    pub async fn serve(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router).await
    }
}

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
        let router = Router::new()
            .with_state(pool.clone())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<Body>| {
                        tracing::span!(
                            Level::TRACE,
                            "request",
                            method = %request.method(),
                            uri = %request.uri(),
                            request_id = tracing::field::Empty,
                        )
                    })
                    .on_request(|request: &Request<Body>, span: &tracing::Span| {
                        if let Some(request_id) = request
                            .headers()
                            .get("x-request-id")
                            .and_then(|v| v.to_str().ok())
                        {
                            span.record("request_id", &request_id);
                        }
                    }),
            )
            .layer(PropagateRequestIdLayer::new(HeaderName::from_static(
                "x-request-id",
            )))
            .layer(SetRequestIdLayer::new(
                HeaderName::from_static("x-request-id"),
                MakeRequestUuid,
            ));
        let listener = TcpListener::bind(format!("{}:{}", args.address, args.port)).await?;

        Ok(Server {
            args: args,
            logs: logs,
            router: router,
            listener: listener,
        })
    }

    pub async fn build_and_serve(self) -> Result<(), Error> {
        let server = self.build().await?;
        let _ = server.serve().await?;

        Ok(())
    }
}
