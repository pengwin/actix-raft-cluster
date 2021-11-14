mod common;
use std::time::Duration;

use actix_web::http::uri::Scheme;
use tokio::select;
use tracing::info_span;

use actix_rt::System;

#[cfg(test)]
use pretty_assertions::assert_eq;

use common::setup_tracing;

use http_json_transport::{create_server, Client, Config};

#[test]
fn health_check() {
    setup_tracing();
    let span = info_span!("test_main");
    let _enter = span.enter();

    let name = "Test Server";
    let cfg = Config::new(name, Scheme::HTTP, "127.0.0.1", 8080);

    let client = Client::new(&cfg).expect("Creating client");

    let sys = System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .worker_threads(2)
            .thread_name("test_thread")
            .build()
            .map_err(|e| format!("Start rt {:?}", e))
            .expect("Building Tokio RT")
    });

    sys.block_on(async move {
        let srv = create_server(&cfg, 1, Duration::from_secs(2)).expect("Creating server");

        select! {
            v = srv => match v {
                Ok(_) => panic!("Server stopped"),
                Err(e) => panic!("Server error {:?}", e)
            },
            v = client.health_check() => match v {
                Ok(res) => {
                    assert_eq!(name, res);
                },
                Err(e) => {
                    panic!("Request error {:?}", e)
                }
            }
        }
    })
}
