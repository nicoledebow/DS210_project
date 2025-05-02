use std::fs::File;
use std::io::{BufRead, BufReader};

/// Loads edges from the dataset into a list of (u32, u32) tuples
pub fn load_edges_from_file(path: &str) -> Vec<(u32, u32)> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut edges = Vec::new();
    let mut skipped = 0;

    for line in reader.lines() {
        if let Ok(l) = line {
            if l.starts_with('#') || l.trim().is_empty() {
                skipped += 1;
                continue;
            }

            let parts: Vec<&str> = l.trim().split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(a), Ok(b)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    edges.push((a, b));
                }
            }
        }
    }

    println!("Skipped {} comment/empty lines.", skipped);
    edges
}
