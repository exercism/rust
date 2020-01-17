use dot_dsl::graph::graph_items::edge::Edge;
use dot_dsl::graph::graph_items::node::Node;
use dot_dsl::graph::Graph;
use maplit::hashmap;

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

    let expected_attrs = hashmap! {
        "foo".to_string() => "1".to_string(),
    };

    assert!(graph.nodes.is_empty());

    assert!(graph.edges.is_empty());

    assert_eq!(graph.attrs, expected_attrs);
}

#[test]
#[ignore]
fn test_graph_with_attributes() {
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

    let expected_attrs = hashmap! {
        "foo".to_string() => "1".to_string(),
        "title".to_string() => "Testing Attrs".to_string(),
        "bar".to_string() => "true".to_string(),
    };

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
fn test_graph_stores_attributes() {
    let attributes = [("foo", "bar"), ("bat", "baz"), ("bim", "bef")];
    let graph = Graph::new().with_nodes(
        &["a", "b", "c"]
            .iter()
            .zip(attributes.iter())
            .map(|(name, &attr)| Node::new(&name).with_attrs(&[attr]))
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        graph
            .get_node("c")
            .expect("node must be stored")
            .get_attr("bim"),
        Some("bef")
    );
}
