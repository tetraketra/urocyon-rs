use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    Router,
    body::Body,
    extract::Request,
    http::{HeaderName, Uri},
    middleware::Next,
    response::Response,
};
use futures_util::future::BoxFuture;
use tower::{Layer, Service};
use tower_http::{
    classify::ServerErrorsFailureClass,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace,
    trace::TraceLayer,
};
use tracing::{Instrument, Level, info_span};

use crate::urocyon::args::LogLevel;

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct RequestContext {
    pub id: String,
    pub uri: String,
    pub method: String,
}

#[allow(dead_code)]
impl RequestContext {
    pub fn log(
        self,
        level: Level,
        msg: &str,
    ) {
        match level {
            /* You can't use `event!` directly here because `level` isn't a literal T>T */
            Level::TRACE => tracing::trace!(msg, id=%self.id, uri=%self.uri, method=%self.method),
            Level::DEBUG => tracing::debug!(msg, id=%self.id, uri=%self.uri, method=%self.method),
            Level::INFO => tracing::info!(msg, id=%self.id, uri=%self.uri, method=%self.method),
            Level::WARN => tracing::warn!(msg, id=%self.id, uri=%self.uri, method=%self.method),
            Level::ERROR => tracing::error!(msg, id=%self.id, uri=%self.uri, method=%self.method),
        };
    }

    pub fn log_trace(
        self,
        msg: &str,
    ) {
        tracing::trace!(msg, id=%self.id, uri=%self.uri, method=%self.method)
    }

    pub fn log_debug(
        self,
        msg: &str,
    ) {
        tracing::debug!(msg, id=%self.id, uri=%self.uri, method=%self.method)
    }

    pub fn log_info(
        self,
        msg: &str,
    ) {
        tracing::info!(msg, id=%self.id, uri=%self.uri, method=%self.method)
    }

    pub fn log_warn(
        self,
        msg: &str,
    ) {
        tracing::warn!(msg, id=%self.id, uri=%self.uri, method=%self.method)
    }

    pub fn log_error(
        self,
        msg: &str,
    ) {
        tracing::error!(msg, id=%self.id, uri=%self.uri, method=%self.method)
    }

    async fn injector(
        mut request: Request<Body>,
        next: Next,
    ) -> Response {
        let uri = request.uri().clone().to_string();
        let method = request.method().clone().to_string();
        let id = request
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let span = tracing::span!(
            Level::TRACE,
            "http_request",
            id = %id,
            uri = %uri,
            method = %method,
            status = tracing::field::Empty,
            duration_ms = tracing::field::Empty,
        );

        request.extensions_mut().insert(RequestContext {
            id: id.clone(),
            uri: uri.clone(),
            method: method.clone(),
        });

        async move {
            let start = std::time::Instant::now();

            tracing::info!(id = %id, uri = %uri, method = %method, "Request started.");

            let response = next.run(request).await;

            let status = response.status().as_u16();
            let duration_ms = start.elapsed().as_secs_f64() * 1000.0;

            tracing::info!(
                id = %id, uri = %uri, method = %method,
                status = status, duration_ms = duration_ms,
                "Request Finished."
            );

            let cur_span = tracing::Span::current();
            cur_span.record("status", &status);
            cur_span.record("duration_ms", &duration_ms);

            response
        }
        .instrument(span)
        .await
    }
}

pub trait RequestContextExtension {
    fn with_trace_layer(self) -> Self;
}

impl RequestContextExtension for Router {
    fn with_trace_layer(self) -> Self {
        self.layer(axum::middleware::from_fn(RequestContext::injector))
    }
}
