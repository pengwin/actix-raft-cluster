mod config;
mod tracing;

use ::tracing::{info, info_span};
use actix_rt::System;

use crate::config::{ClusterConfig, NodeArgConfig};

use cluster_node::{ClusterNode, NodeError};
use std::io::{Error, ErrorKind};

fn main() -> std::io::Result<()> {
    crate::tracing::setup_tracing()?;

    let cli_config = NodeArgConfig::read_from_args();
    let file_config = ClusterConfig::from_file(cli_config.config_file.as_str())?;

    let span = info_span!("app_main", config = cli_config.to_string().as_str());
    let _enter = span.enter();

    info!("File config {:?}", file_config);

    let node_config = cli_config.node_config(&file_config)?;

    let sys = System::new();
    sys.block_on(async {
        let node = ClusterNode::new(&node_config)?;
        node.run().await
    })
    .map_err(to_io)
}

fn to_io(e: NodeError) -> Error {
    Error::new(ErrorKind::Other, format!("{:?}", e))
}
