mod graph;
mod utils;

use graph::ProductGraph;
use utils::load_edges_from_file;

fn main() {
    let path = "amazon0601.txt";
    let edges = load_edges_from_file(path);

    println!("Loaded {} edges", edges.len());

    let graph = ProductGraph::from_edges(&edges);
    let centrality = graph.degree_centrality();

    let mut sorted: Vec<_> = centrality.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    println!("Top 5 most central products:");
    for (product, degree) in sorted.iter().take(5) {
        println!("Product {} with {} connections", product, degree);
    }

    println!("Total nodes: {}", graph.node_count());
    println!("Total edges: {}", graph.edge_count());
    println!("Isolated nodes: {}", graph.isolated_nodes().len());
    println!("Average degree: {:.2}", graph.average_degree());
}
