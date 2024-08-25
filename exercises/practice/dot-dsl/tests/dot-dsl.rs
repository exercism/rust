use std::collections::HashMap;

use dot_dsl::graph::graph_items::edge::Edge;
use dot_dsl::graph::graph_items::node::Node;
use dot_dsl::graph::Graph;

#[test]
fn empty_graph() {
    let graph = Graph::new();

    assert!(graph.nodes.is_empty());

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());
}

#[test]
#[ignore]
fn graph_with_one_node() {
    let nodes = vec![Node::new("a")];

    let graph = Graph::new().with_nodes(&nodes);

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(graph.nodes, vec![Node::new("a")]);
}

#[test]
#[ignore]
fn graph_with_one_node_with_keywords() {
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
fn graph_with_one_edge() {
    let edges = vec![Edge::new("a", "b")];

    let graph = Graph::new().with_edges(&edges);

    assert!(graph.nodes.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(graph.edges, vec![Edge::new("a", "b")]);
}

#[test]
#[ignore]
fn graph_with_one_edge_with_keywords() {
    let edges = vec![Edge::new("a", "b").with_attrs(&[("color", "blue")])];

    let graph = Graph::new().with_edges(&edges);

    assert!(graph.nodes.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(
        graph.edges,
        vec![Edge::new("a", "b").with_attrs(&[("color", "blue")])]
    );
}

#[test]
#[ignore]
fn graph_with_one_attribute() {
    let graph = Graph::new().with_attrs(&[("foo", "1")]);

    let expected_attrs = HashMap::from([("foo".to_string(), "1".to_string())]);

    assert!(graph.nodes.is_empty());

    assert!(graph.edges.is_empty());

    assert_eq!(graph.attrs, expected_attrs);
}

#[test]
#[ignore]
fn graph_with_attributes() {
    let nodes = vec![
        Node::new("a").with_attrs(&[("color", "green")]),
        Node::new("c"),
        Node::new("b").with_attrs(&[("label", "Beta!")]),
    ];

    let edges = vec![
        Edge::new("b", "c"),
        Edge::new("a", "b").with_attrs(&[("color", "blue")]),
    ];

    let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];

    let expected_attrs = HashMap::from([
        ("foo".to_string(), "1".to_string()),
        ("title".to_string(), "Testing Attrs".to_string()),
        ("bar".to_string(), "true".to_string()),
    ]);

    let graph = Graph::new()
        .with_nodes(&nodes)
        .with_edges(&edges)
        .with_attrs(&attrs);

    assert_eq!(
        graph.nodes,
        vec![
            Node::new("a").with_attrs(&[("color", "green")]),
            Node::new("c"),
            Node::new("b").with_attrs(&[("label", "Beta!")]),
        ]
    );

    assert_eq!(
        graph.edges,
        vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue")]),
        ]
    );

    assert_eq!(graph.attrs, expected_attrs);
}

#[test]
#[ignore]
fn edges_store_attributes() {
    let nodes = vec![
        Node::new("a").with_attrs(&[("color", "green")]),
        Node::new("c"),
        Node::new("b").with_attrs(&[("label", "Beta!")]),
    ];

    let edges = vec![
        Edge::new("b", "c"),
        Edge::new("a", "b").with_attrs(&[("color", "blue"), ("fill", "darkblue")]),
    ];

    let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];

    let graph = Graph::new()
        .with_nodes(&nodes)
        .with_edges(&edges)
        .with_attrs(&attrs);

    assert_eq!(
        graph.edges,
        vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue"), ("fill", "darkblue")]),
        ]
    );

    assert_eq!(graph.edges[1].attr("color"), Some("blue"));
    assert_eq!(graph.edges[1].attr("fill"), Some("darkblue"));
    assert_eq!(graph.edges[1].attr("foo"), None);
    assert_eq!(graph.edges[0].attr("color"), None);
    assert_eq!(graph.edges[0].attr("fill"), None);
    assert_eq!(graph.edges[0].attr("foo"), None);
}

#[test]
#[ignore]
fn graph_nodes_store_attributes() {
    let attributes = [("foo", "bar"), ("bat", "baz"), ("bim", "bef")];
    let graph = Graph::new().with_nodes(
        &["a", "b", "c"]
            .iter()
            .zip(attributes.iter())
            .map(|(name, &attr)| Node::new(name).with_attrs(&[attr]))
            .collect::<Vec<_>>(),
    );

    let a = graph.node("a").expect("node a must be stored");
    assert_eq!(a.attr("foo"), Some("bar"));
    assert_eq!(a.attr("bat"), None);
    assert_eq!(a.attr("bim"), None);

    let b = graph.node("b").expect("node b must be stored");
    assert_eq!(b.attr("foo"), None);
    assert_eq!(b.attr("bat"), Some("baz"));
    assert_eq!(b.attr("bim"), None);

    let c = graph.node("c").expect("node c must be stored");
    assert_eq!(c.attr("foo"), None);
    assert_eq!(c.attr("bat"), None);
    assert_eq!(c.attr("bim"), Some("bef"));
}
