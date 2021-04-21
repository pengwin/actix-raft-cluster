//use tracing_subscriber::fmt::time::ChronoUtc;

//const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

//use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::{SubscriberInitExt, TryInitError};
use std::io::{Result, Error, ErrorKind};

use crate::config::ClusterConfig;

pub fn setup_tracing(_config: &ClusterConfig) -> Result<()> {

    /*let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(format!("cluster_node_{}", config.node_id))
        .install_simple()?;
    
    let opentelemetry = tracing_opentelemetry::layer()
        .with_tracer(tracer);*/

    let fmt = tracing_subscriber::fmt::layer()
        //.pretty()
        .compact()
        //.with_timer(ChronoUtc::with_format(TIME_FORMAT.to_owned()))
        .without_time();
        //.with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        //.with_thread_ids(true)
        //.with_thread_names(true);

    tracing_subscriber::registry()
        //.with(opentelemetry)
        .with(fmt)
        .try_init()
        .map_err(to_io)
}

fn to_io(e: TryInitError) -> Error {
    Error::new(ErrorKind::Other, format!("Tracing Init Error {:?}", e))
}