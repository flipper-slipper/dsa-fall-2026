mod dijkstra;
mod euler81;
mod graph;
mod min_heap;
mod priority_queue;

use dijkstra::shortest_path;
use graph::Graph;

fn main() {
    // Each road is two-way, so they are added in both directions.
    let roads = [
        ("Boston", "Needham", 13.0),
        ("Boston", "Framingham", 21.0),
        ("Boston", "Worcester", 47.0),
        ("Boston", "Providence", 50.0),
        ("Needham", "Framingham", 12.0),
        ("Needham", "Providence", 40.0),
        ("Framingham", "Worcester", 22.0),
        ("Worcester", "Providence", 43.0),
        ("Worcester", "Springfield", 53.0),
    ];

    let mut graph = Graph::new();
    for (a, b, miles) in roads {
        graph.add_edge(a, b, miles);
        graph.add_edge(b, a, miles);
    }
    // Add an edge case of Island to Boston only.
    graph.add_edge("Nantucket", "Boston", 100.0);

    // Find the shortest path for each of these trips, where the last one should be impossible.
    let trips = [
        ("Boston", "Springfield", Some(vec!["Boston", "Framingham", "Worcester", "Springfield"])),
        ("Providence", "Framingham", Some(vec!["Providence", "Needham", "Framingham"])),
        ("Springfield", "Needham", Some(vec!["Springfield", "Worcester", "Framingham", "Needham"])),
        ("Boston", "Boston", Some(vec!["Boston"])),
        ("Nantucket", "Needham", Some(vec!["Nantucket", "Boston", "Needham"])),
        ("Boston", "Nantucket", None),
    ];
    for (start, target, expected) in trips {
        assert_eq!(shortest_path(&graph, &start, &target), expected, "{start} -> {target}");
    }

    // Euler Problem 81.
    let matrix = vec![
        vec![131.0, 673.0, 234.0, 103.0, 18.0],
        vec![201.0, 96.0, 342.0, 965.0, 150.0],
        vec![630.0, 803.0, 746.0, 422.0, 111.0],
        vec![537.0, 699.0, 497.0, 121.0, 956.0],
        vec![805.0, 732.0, 524.0, 37.0, 331.0],
    ];
    let sum = euler81::min_path_sum(&matrix);
    println!("Euler 81 minimal path sum: {sum}");
    assert_eq!(sum, 2427.0);
}
