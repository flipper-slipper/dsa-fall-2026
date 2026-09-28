use std::collections::HashMap;
use std::hash::Hash;

use crate::graph::Graph;
use crate::priority_queue::PriorityQueue;

/// Finds the shortest path from `start` to `target` using Dijkstra's
/// algorithm. Edge weights must be non-negative.
///
/// Returns the vertices along the path, including `start` and `target`, or
/// `None` if `target` can't be reached from `start`.
pub fn shortest_path<V: Eq + Hash + Clone>(graph: &Graph<V>, start: &V, target: &V) -> Option<Vec<V>> {
    // The shortest known distance to each vertex reached so far
    let mut dist: HashMap<V, f64> = HashMap::new();
    // The vertex before each vertex on its shortest known path
    let mut prev: HashMap<V, V> = HashMap::new();
    let mut queue = PriorityQueue::new();

    dist.insert(start.clone(), 0.0);
    queue.add_with_priority(start.clone(), 0.0);

    while let Some(current) = queue.next() {
        // Leave early
        if current == *target {
            break;
        }

        let current_dist = dist[&current];
        for (neighbor, cost) in graph.get_edges(&current) {
            let new_dist = current_dist + cost;
            match dist.get(&neighbor) {
                // Already have a path at least this short
                Some(&old_dist) if old_dist <= new_dist => continue,
                // Found a shorter path to a vertex that's still in the queue
                Some(_) => queue.adjust_priority(&neighbor, new_dist),
                // First time reaching this vertex
                None => queue.add_with_priority(neighbor.clone(), new_dist),
            }
            dist.insert(neighbor.clone(), new_dist);
            prev.insert(neighbor, current.clone());
        }
    }

    if !dist.contains_key(target) {
        return None;
    }

    // Walk backwards from the target to the start, then reverse
    let mut path: Vec<V> = std::iter::successors(Some(target.clone()), |v| prev.get(v).cloned()).collect();
    path.reverse();
    Some(path)
}
