pub struct ConsoleConfig {
    pub enabled: bool,
    pub pretty_print: bool,
    /*pub with_thread_id: bool,
    pub with_thread_name: bool,
    pub with_span_events: bool,*/
}

pub struct JaegerConfig {
    pub enabled: bool,
    pub service_name: &'static str,
}

pub struct TracingConfig {
    pub console_config: Option<ConsoleConfig>,
    pub jaeger_config: Option<JaegerConfig>,
}