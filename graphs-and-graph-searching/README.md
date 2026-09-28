# Assignment 3: Graph Searching and Shortest Paths

## Where things are

- `src/graph.rs`: directed, weighted graph (`Graph`)
- `src/min_heap.rs`: min heap, implemented from scratch (`MinHeap`)
- `src/priority_queue.rs`: priority queue backed by the min heap (`PriorityQueue`)
- `src/dijkstra.rs`: Dijkstra's algorithm (`shortest_path`)
- `src/euler81.rs`: Project Euler Problem 81 solved with Dijkstra
- `src/main.rs`: runs a city map demo and Project Euler 81 on the 5x5 example matrix

## How to run

From the repository root:

```sh
cargo run -p graphs-and-graph-searching    # run the demos
cargo test -p graphs-and-graph-searching   # run the unit tests
```
