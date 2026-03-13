use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, HeaderValue},
    middleware::Next,
    response::Response,
};
use opentelemetry::{
    global,
    propagation::Extractor,
    trace::{TraceContextExt, TracerProvider},
};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator, trace::TracerProvider as SdkTracerProvider,
};
use tracing::Instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub const TRACE_ID_HEADER: &str = "x-liberte-trace-id";

#[derive(Clone, Debug)]
pub struct TraceContext {
    pub trace_id: String,
}

struct HeaderExtractor<'a>(&'a HeaderMap);

impl Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|key| key.as_str()).collect()
    }
}

pub fn init_tracing(service_name: &'static str) {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let provider = SdkTracerProvider::builder().build();
    let tracer = provider.tracer(service_name);
    global::set_tracer_provider(provider);

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
}

pub async fn trace_http_request(mut request: Request<Body>, next: Next) -> Response {
    let parent_context = global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(request.headers()))
    });

    let span = tracing::info_span!(
        "http.request",
        method = %request.method(),
        uri = %request.uri(),
        trace_id = tracing::field::Empty
    );
    span.set_parent(parent_context);

    let trace_id = span.context().span().span_context().trace_id().to_string();
    span.record("trace_id", tracing::field::display(&trace_id));
    request.extensions_mut().insert(TraceContext {
        trace_id: trace_id.clone(),
    });

    let mut response = next.run(request).instrument(span).await;
    if let Ok(value) = HeaderValue::from_str(&trace_id) {
        response.headers_mut().insert(TRACE_ID_HEADER, value);
    }
    response
}
