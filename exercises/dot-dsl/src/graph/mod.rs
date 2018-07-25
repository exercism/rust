pub mod graph_items;

use std::collections::HashMap;

use self::graph_items::{Edge, Node};

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.clone().to_vec();

        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.clone().to_vec();

        self
    }

    pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
        attrs.iter().for_each(|&(name, value)| {
            self.attrs.insert(name.to_string(), value.to_string());
        });

        self
    }
}
