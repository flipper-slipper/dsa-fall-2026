use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// A directed, weighted graph.
pub struct Graph<V> {
    /// An adjaceny list of of the Nodes and its connected Nodes and Weights.
    adjacency: HashMap<V, HashMap<V, f64>>,
}

impl<V: Eq + Hash + Clone> Graph<V> {
    /// Creates an empty graph.
    pub fn new() -> Self {
        Graph { adjacency: HashMap::new() }
    }

    /// Returns the vertices in the graph.
    pub fn get_vertices(&self) -> HashSet<V> {
        self.adjacency.keys().cloned().collect()
    }

    /// Add an edge between `from` and `to` with edge weight `cost`.
    pub fn add_edge(&mut self, from: V, to: V, cost: f64) {
        self.adjacency.entry(from).or_default().insert(to.clone(), cost);
        // Add `to` itself to the adjacency 
        self.adjacency.entry(to).or_default();
    }

    /// Get all the edges that begin at `from`.
    ///
    /// Returns a map where each key represents a vertex connected to `from`
    /// and the value represents the edge weight.
    pub fn get_edges(&self, from: &V) -> HashMap<V, f64> {
        // A vertex that isn't in the graph has no edges
        self.adjacency.get(from).cloned().unwrap_or_default()
    }

    /// Remove all edges and vertices from the graph.
    pub fn clear(&mut self) {
        self.adjacency.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_graph_is_empty() {
        let graph: Graph<&str> = Graph::new();
        assert!(graph.get_vertices().is_empty());
    }

    #[test]
    fn get_vertices_returns_all_vertices() {
        let mut graph = Graph::new();
        graph.add_edge("A", "B", 1.0);
        assert_eq!(graph.get_vertices(), HashSet::from(["A", "B"]));
    }

    #[test]
    fn add_edge_adds_directed_edge() {
        let mut graph = Graph::new();
        graph.add_edge("A", "B", 1.0);
        assert_eq!(graph.get_edges(&"A"), HashMap::from([("B", 1.0)]));
        assert!(graph.get_edges(&"B").is_empty());
    }

    #[test]
    fn get_edges_returns_outgoing_edges() {
        let mut graph = Graph::new();
        graph.add_edge("A", "B", 1.0);
        graph.add_edge("A", "C", 2.0);
        assert_eq!(graph.get_edges(&"A"), HashMap::from([("B", 1.0), ("C", 2.0)]));
    }

    #[test]
    fn clear_removes_everything() {
        let mut graph = Graph::new();
        graph.add_edge("A", "B", 1.0);
        graph.clear();
        assert!(graph.get_vertices().is_empty());
    }
}
