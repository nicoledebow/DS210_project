use std::collections::{HashMap, HashSet};

/// Represents a product co-purchase graph
pub struct ProductGraph {
    pub adjacency_list: HashMap<u32, HashSet<u32>>,
}

impl ProductGraph {
    pub fn from_edges(edges: &[(u32, u32)]) -> Self {
        let mut graph = ProductGraph {
            adjacency_list: HashMap::new(),
        };

        for &(a, b) in edges {
            graph.adjacency_list.entry(a).or_default().insert(b);
            graph.adjacency_list.entry(b).or_default().insert(a);
        }

        graph
    }

    pub fn degree_centrality(&self) -> HashMap<u32, usize> {
        self.adjacency_list
            .iter()
            .map(|(&node, neighbors)| (node, neighbors.len()))
            .collect()
    }

    pub fn node_count(&self) -> usize {
        self.adjacency_list.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adjacency_list
            .values()
            .map(|neighbors| neighbors.len())
            .sum::<usize>() / 2
    }

    pub fn isolated_nodes(&self) -> Vec<u32> {
        self.adjacency_list
            .iter()
            .filter(|(_, neighbors)| neighbors.is_empty())
            .map(|(&node, _)| node)
            .collect()
    }

    /// Returns the average degree (connections per node)
    pub fn average_degree(&self) -> f64 {
        if self.adjacency_list.is_empty() {
            return 0.0;
        }
        let total_degrees: usize = self
            .adjacency_list
            .values()
            .map(|neighbors| neighbors.len())
            .sum();
        total_degrees as f64 / self.adjacency_list.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_construction() {
        let edges = vec![(1, 2), (2, 3), (3, 1)];
        let graph = ProductGraph::from_edges(&edges);
        assert_eq!(graph.adjacency_list.get(&1).unwrap().len(), 2);
        assert_eq!(graph.adjacency_list.get(&2).unwrap().len(), 2);
    }

    #[test]
    fn test_degree_centrality() {
        let edges = vec![(1, 2), (2, 3), (3, 4)];
        let graph = ProductGraph::from_edges(&edges);
        let centrality = graph.degree_centrality();
        assert_eq!(centrality.get(&2), Some(&2));
    }

    #[test]
    fn test_isolated_nodes() {
        let edges = vec![(1, 2), (3, 4)];
        let mut graph = ProductGraph::from_edges(&edges);
        graph.adjacency_list.insert(5, HashSet::new());
        let isolated = graph.isolated_nodes();
        assert!(isolated.contains(&5));
    }

    #[test]
    fn test_average_degree() {
        let edges = vec![(1, 2), (2, 3)];
        let graph = ProductGraph::from_edges(&edges);
        let avg = graph.average_degree();
        assert!(avg >= 1.0 && avg <= 2.0);
    }
}