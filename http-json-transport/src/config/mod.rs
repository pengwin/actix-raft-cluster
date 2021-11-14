use actix_web::http::uri::Scheme;

#[derive(Debug)]
pub struct Config {
    pub(super) scheme: Scheme,
    pub(super) name: String,
    pub(super) client_endpoint: String,
    pub(super) server_endpoint: String,
}

impl Config {
    pub fn new(name: &str, scheme: Scheme, host: &str, port: u16) -> Config {
        let client_endpoint = format!("{}://{}:{}", scheme.as_str(), host, port);
        Config {
            scheme,
            name: name.to_owned(),
            client_endpoint,
            server_endpoint: format!("{}:{}", host, port),
        }
    }
}
