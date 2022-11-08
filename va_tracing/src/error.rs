use opentelemetry::trace::TraceError;
use thiserror::Error;
use tracing_subscriber::util::TryInitError;

#[derive(Error, Debug)]
pub enum TracingError {
    #[error("Try Init Error")]
    TryInitError(#[from]TryInitError),
    #[error("JaegerTraceError")]
    JaegerTraceError(#[from]TraceError)
}