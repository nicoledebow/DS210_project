mod graph;
mod utils;

use graph::*;
use utils::*;

fn main() {
    let path = "amazon0601.txt"; // Make sure the file is in your project root
    let edges = load_edges_from_file(path);

    println!("Loaded {} edges", edges.len());

    let graph = ProductGraph::from_edges(&edges);
    let centrality = graph.degree_centrality();

    let mut sorted: Vec<_> = centrality.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1)); // sort descending by degree

    println!("Top 5 most central products:");
    for (product, degree) in sorted.iter().take(5) {
        println!("Product {} with {} connections", product, degree);
    }
}
