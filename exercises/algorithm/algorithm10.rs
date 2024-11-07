use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>, // 邻接表
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        self.add_node(from);
        self.add_node(to);

        // 添加无向边，避免重复
        if !self.contains_edge(from, to, weight) {
            self.adjacency_table_mutable()
                .entry(from.to_string())
                .or_default()
                .push((to.to_string(), weight));
            self.adjacency_table_mutable()
                .entry(to.to_string())
                .or_default()
                .push((from.to_string(), weight));
        }
    }

    fn contains_edge(&self, from: &str, to: &str, weight: i32) -> bool {
        if let Some(neighbours) = self.adjacency_table().get(from) {
            for (neighbor, w) in neighbours {
                if neighbor == to && *w == weight {
                    return true;
                }
            }
        }
        false
    }
}

pub trait Graph {
    fn new() -> Self;

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        if self.contains(node) {
            false // 如果节点已经存在，返回 false
        } else {
            self.adjacency_table_mutable()
                .insert(node.to_string(), Vec::new());
            true
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32));

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn contains_edge(&self, from: &str, to: &str, weight: i32) -> bool;

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(String, String, i32)> {
        let mut edges = Vec::new();
        let mut seen_edges = HashSet::new();

        for (from_node, neighbours) in self.adjacency_table() {
            for (to_node, weight) in neighbours {
                // 确保无向边只记录一次
                let edge = if from_node < to_node {
                    (from_node.clone(), to_node.clone(), *weight)
                } else {
                    (to_node.clone(), from_node.clone(), *weight)
                };

                if !seen_edges.contains(&(edge.0.clone(), edge.1.clone())) {
                    edges.push(edge.clone());
                    seen_edges.insert((edge.0, edge.1));
                }
            }
        }

        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::{Graph, UndirectedGraph};

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = vec![
            ("a".to_string(), "b".to_string(), 5),
            ("a".to_string(), "c".to_string(), 7),
            ("b".to_string(), "c".to_string(), 10),
        ];

        let graph_edges = graph.edges();
        for edge in expected_edges {
            assert!(graph_edges.contains(&edge), "Missing edge: {:?}", edge);
        }
    }
}

