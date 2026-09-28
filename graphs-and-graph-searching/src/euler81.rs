use crate::dijkstra::shortest_path;
use crate::graph::Graph;

/// Finds shortest path.
pub fn min_path_sum(matrix: &[Vec<f64>]) -> f64 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // Populate the graph from the matrix. 
    let mut graph = Graph::new();
    for r in 0..rows {
        for c in 0..cols {
            if c + 1 < cols {
                graph.add_edge((r, c), (r, c + 1), matrix[r][c + 1]);
            }
            if r + 1 < rows {
                graph.add_edge((r, c), (r + 1, c), matrix[r + 1][c]);
            }
        }
    }

    let path = shortest_path(&graph, &(0, 0), &(rows - 1, cols - 1)).unwrap();
    // Sum the paths,
    path.iter().map(|&(r, c)| matrix[r][c]).sum()
}
