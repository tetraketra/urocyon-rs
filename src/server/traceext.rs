use axum::{
    Router,
    body::Body,
    http::{HeaderName, Request, Response},
};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace,
};
use tracing::Level;

pub trait TraceExtension {
    fn with_trace_layer(self) -> Self;
}

impl TraceExtension for Router {
    fn with_trace_layer(self) -> Self {
        let header = HeaderName::from_static("x-request-id");
        let trace_layer = trace::TraceLayer::new_for_http()
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
            });

        self.layer(trace_layer)
            .layer(PropagateRequestIdLayer::new(header.clone()))
            .layer(SetRequestIdLayer::new(header.clone(), MakeRequestUuid))
    }
}
