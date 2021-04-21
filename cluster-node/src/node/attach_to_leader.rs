use node_actor::{RemoteNodeActorAddr, AttachNode};
use crate::config::RemoteNodeConfig;
use crate::node::error::AttachError;
use actor_registry::NodesRegistry;

pub async fn attach_to_leader(
    nodes: NodesRegistry,
    leader_node: &RemoteNodeConfig,
    this_node: &RemoteNodeConfig,
) -> Result<bool, AttachError> {
    let this_node = this_node.clone();

    let addr = leader_node.addr().clone();
    let remote_leader = RemoteNodeActorAddr::new(leader_node.node_id, addr.clone());

    let _ = remote_leader
        .send(&AttachNode::new(this_node.node_id, this_node.addr()))
        .await
        .map_err(AttachError::from)?
        .map_err(AttachError::from)?;

    nodes.attach_node(leader_node.node_id, &addr).await;

    Ok(true)
}
