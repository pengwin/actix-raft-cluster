mod config;
mod tracing;

use std::sync::Arc;

use actix_rt::System;
use ::tracing::info_span;

use crate::config::ClusterConfig;

use cluster_node::{ClusterNode, NodeError};
use std::io::{Error, ErrorKind};

fn main() -> std::io::Result<()> {
    let cli_config = ClusterConfig::read_from_args();
    crate::tracing::setup_tracing(&cli_config)?;

    let span = info_span!("app_main", config = cli_config.to_string().as_str());
    let _enter = span.enter();

    let sys = System::new();
    sys.block_on(async {
        let node = ClusterNode::new(Arc::new(cli_config.node_config()))?;
        node.run().await
    })
    .map_err(to_io)
}

fn to_io(e: NodeError) -> Error {
    Error::new(ErrorKind::Other, format!("{:?}", e))
}
