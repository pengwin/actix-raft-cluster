use std::sync::Once;
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;
#[cfg(test)]
use tracing_subscriber::layer::SubscriberExt;
#[cfg(test)]
use tracing_subscriber::util::SubscriberInitExt;


static INIT: Once = Once::new();

pub fn setup_tracing() {
    INIT.call_once(internal_setup_tracing);
}

fn internal_setup_tracing() {
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    let fmt = tracing_subscriber::fmt::layer()
        .compact()
        .without_time();

    tracing_subscriber::registry().with(fmt).with(filter).init();
}
