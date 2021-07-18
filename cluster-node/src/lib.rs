//! Cluster of nodes with actors
//!
//! Actors powered by [actix framework](https://github.com/actix/actix)
//!
//! Network layer powered by [actix-web](https://github.com/actix/actix-web)
//!
//! The end goal is to implement cluster of virtual actors similar to [Orleans](https://dotnet.github.io/orleans/)
//!
//! Features plan:
//!
//! - **✓ Actors can be reached remotely**
//! - **✓ Actors activated by first call**
//! -  Actors passivation after inactivity period
//! - **✓ Cluster nodes can attach to leader**
//! -  Cluster nodes ping each other and detach
//! -  Cluster nodes detach unreachable nodes
//! -  Use raft protocol
//! - **✓ Nodes use simple http server for network communication**
//! -  Nodes use tcp server for network communication
//! -  Builder like API to register actor types

#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]
#![doc(html_no_source)]

mod config;
mod node;
mod web_server;

pub use config::{NodeConfig, RemoteNodeConfig, RemoteNodeConfigProtocol};
pub use node::{AttachError, ClusterNode, NodeError};
