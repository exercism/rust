extern crate dot_dsl;

use dot_dsl::{Edge, Graph, Node};
use std::collections::HashMap;

#[test]
fn test_empty_graph() {
    let graph = Graph::new();

    assert!(graph.nodes.is_empty());

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());
}

#[test]
#[ignore]
fn test_graph_with_one_node() {
    let nodes = vec![Node::new("a")];

    let graph = Graph::new().with_nodes(&nodes);

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(graph.nodes, vec![Node::new("a")]);
}

#[test]
#[ignore]
fn test_graph_with_one_node_with_keywords() {
    let nodes = vec![Node::new("a").with_attrs(&[("color", "green")])];

    let graph = Graph::new().with_nodes(&nodes);

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(
        graph.nodes,
        vec![Node::new("a").with_attrs(&[("color", "green")])]
    );
}

#[test]
#[ignore]
fn test_graph_with_one_edge() {
    let edges = vec![Edge::new("a", "b")];

    let graph = Graph::new().with_edges(&edges);

    assert!(graph.nodes.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(graph.edges, vec![Edge::new("a", "b")]);
}

#[test]
#[ignore]
fn test_graph_with_one_attribute() {
    let graph = Graph::new().with_attrs(&[("foo", "1")]);

    let mut expected_attrs = HashMap::new();

    expected_attrs.insert("foo".to_string(), "1".to_string());

    assert!(graph.nodes.is_empty());

    assert!(graph.edges.is_empty());

    assert_eq!(graph.attrs, expected_attrs);
}
