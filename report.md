This project focuses on understanding how products relate to each other in Amazon’s co-purchasing network. The main goal was to identify which products show up most often in co-purchase relationships, and to see if any meaningful structure or communities emerge from those connections. By treating each product as a node and each co-purchase as an undirected edge, I was able to apply basic graph analysis techniques to explore product centrality and connectivity at scale.

The dataset I used comes from Stanford’s SNAP collection, specifically the Amazon Product Co-Purchasing Network. It contains hundreds of thousands of products and nearly a million edges representing products that tend to be bought together. After downloading and unzipping the data, I used the file amazon0601.txt, which is around 47MB. Each line in the file represents a pair of product IDs, so it was straightforward to parse into edges for a graph.

To load the data, I used Rust’s standard library tools like BufReader to efficiently read the file line by line. I skipped comment lines and empty lines, then split each valid line into two integers. These were parsed into (u32, u32) pairs and added to a vector of edges. This list of edges became the input for building my graph structure.

The code is split across three modules. In main.rs, I handled the high-level flow: loading the file, constructing the graph, computing statistics, and printing results. The graph logic lives in graph.rs, where I built a ProductGraph struct using an adjacency list. I implemented methods to compute degree centrality, count the number of nodes and edges, and find isolated nodes with no connections. I also included unit tests to make sure these methods worked as expected. In utils.rs, I wrote a helper function to load the data file and return the cleaned edge list.

The program comfortably hits the 150-line requirement thanks to the modular structure, additional graph statistics, and tests. The test cases in graph.rs use small, hardcoded graphs and check things like correct centrality values and isolated node detection. All tests were run using cargo test and passed successfully.

When running the program on the full dataset, it loads 925,872 edges and builds a graph with over 260,000 nodes. It then sorts products by how many connections they have and prints the top five. These central products are likely bestsellers, core accessories, or general-purpose items that frequently appear in a variety of shopping carts. The output also includes the total number of nodes, edges, and isolated nodes—offering a quick snapshot of the graph’s shape.

To run the program, all that’s needed is to place amazon0601.txt in the root folder and run cargo run --release. The program completes in just a few seconds and outputs a set of stats to the terminal.

While working on the project, I used ChatGPT occasionally to help debug specific Rust errors and to better understand how to structure modules and pass data between files. It was especially helpful when I got stuck on visibility issues between functions or wanted to write quick tests. I still wrote and edited all the code and the report myself, but used the AI as a way to speed things up and double-check parts I wasn’t sure about.

The dataset was provided by the Stanford SNAP group and is publicly available at https://snap.stanford.edu/data/amazon0601.html.
