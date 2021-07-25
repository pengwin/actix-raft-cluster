use cluster_config::{ClusterNodesConfig, NodeItem};
use std::thread;

#[test]
fn config_all_node_ids_thread() {
    // arrange
    let nodes = vec![NodeItem::new(1, "http://127.0.0.1"), NodeItem::new(2, "http://127.0.0.1")];
    let config = ClusterNodesConfig::new(1, &nodes);
    let factory = config.factory();
    let handle_one = factory.create();
    let handle_two = factory.create();
    
    // act 
    let all_node_ids_one = thread::spawn(move || {
        handle_one.all_node_ids()
    }).join().expect(" Error join thread");
    let all_node_ids_two = thread::spawn(move || {
        handle_two.all_node_ids()
    }).join().expect(" Error join thread");
    
    // assert
    assert_eq!(2, all_node_ids_one.len());
    assert_eq!(2, all_node_ids_two.len());
}

#[test]
fn config_all_node_ids() {
    // arrange
    let nodes = vec![NodeItem::new(1, "http://127.0.0.1"), NodeItem::new(2, "http://127.0.0.1")];
    let config = ClusterNodesConfig::new(1, &nodes);
    let factory = config.factory();
    let handle = factory.create();

    // act
    let all_node_ids = handle.all_node_ids();

    // assert
    assert_eq!(2, all_node_ids.len());
    assert_eq!(true, all_node_ids.contains(&1));
    assert_eq!(true, all_node_ids.contains(&2));
}

#[test]
fn config_add_node() {
    // arrange
    let nodes = vec![NodeItem::new(1, "http://127.0.0.1"), NodeItem::new(2, "http://127.0.0.1")];
    let mut config = ClusterNodesConfig::new(1, &nodes);
    let factory = config.factory();
    let handle = factory.create();

    // act 
    tokio_test::block_on( config.add_node(NodeItem::new(5, "http://127.0.0.1")));
    let all_node_ids = handle.all_node_ids();

    // assert
    assert_eq!(3, all_node_ids.len());
    assert_eq!(true, all_node_ids.contains(&1));
    assert_eq!(true, all_node_ids.contains(&2));
    assert_eq!(true, all_node_ids.contains(&5));
}