//use tracing_subscriber::fmt::time::ChronoUtc;

//const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

//use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn setup_tracing() {
    let fmt = tracing_subscriber::fmt::layer()
        //.pretty()
        .compact()
        //.with_timer(ChronoUtc::with_format(TIME_FORMAT.to_owned()))
        .without_time();
    //.with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
    //.with_thread_ids(true)
    //.with_thread_names(true);

    tracing_subscriber::registry()
        .with(fmt)
        .init();
}