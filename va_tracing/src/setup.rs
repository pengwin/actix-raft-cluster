use opentelemetry::sdk::trace::Tracer;
use tracing_subscriber::fmt::time::{FormatTime, UtcTime};

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::{SubscriberInitExt, TryInitError};

use crate::config::TracingConfig;
use crate::error::TracingError;

use opentelemetry::sdk::export::trace::stdout;

pub fn setup_tracing(config: TracingConfig) -> Result<(), TracingError> {
    let jaeger_tracer = setup_jaeger(&config)?;
    let console_tracer = setup_console(&config)?;

    let opentelemetry = match (jaeger_tracer, console_tracer) {
        (None, None) => None,
        (None, Some(c_tracer)) => Some(tracing_opentelemetry::layer().with_tracer(c_tracer)),
        (Some(j_tracer), None) => Some(tracing_opentelemetry::layer().with_tracer(j_tracer)),
        (Some(j_tracer), Some(c_tracer)) => Some(
            tracing_opentelemetry::layer()
                .with_tracer(j_tracer)
                .with_tracer(c_tracer),
        ),
    };

    let fmt = tracing_subscriber::fmt::layer().pretty().without_time();

    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(fmt)
        .try_init()?;

    Ok(())
}

fn setup_jaeger(config: &TracingConfig) -> Result<Option<Tracer>, TracingError> {
    if let Some(jaeger_config) = &config.jaeger_config {
        if !jaeger_config.enabled {
            return Ok(None);
        }

        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name(jaeger_config.service_name.to_owned())
            .install_simple()?;

        return Ok(Some(tracer));
    }

    Ok(None)
}

fn setup_console(config: &TracingConfig) -> Result<Option<Tracer>, TracingError> {
    if let Some(console_config) = &config.console_config {
        if !console_config.enabled {
            return Ok(None);
        }

        let tracer = stdout::new_pipeline()
            .with_pretty_print(console_config.pretty_print)
            .install_simple();

        return Ok(Some(tracer));
    }
    Ok(None)
}
