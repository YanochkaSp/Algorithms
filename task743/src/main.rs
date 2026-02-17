#![allow(unused)]
use std::{cmp::Reverse, collections::BinaryHeap};

fn dijkstra(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let start = (k - 1) as usize;

    let mut graph: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];

    for edge in times {
        let u = (edge[0] - 1) as usize;
        let v = (edge[1] - 1) as usize;
        let w = edge[2];
        graph[u].push((v, w));
    }

    let mut dists = vec![i32::MAX; n];
    dists[start] = 0;

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start)));

    while let Some(Reverse((dist, u))) = queue.pop() {
        if dist > dists[u] {
            continue;
        }
        for &(v, w) in &graph[u] {
            if let Some(new_dist) = dists[u].checked_add(w) {
                if new_dist < dists[v] {
                    dists[v] = new_dist;
                    queue.push(Reverse((dists[v], v)));
                }
            }
        }
    }

    let time = *dists.iter().max().expect("Expected non-empty iterator");

    if time == i32::MAX { -1 } else { time }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::dijkstra;

    #[test]
    fn single() {
        assert_eq!(dijkstra(vec![], 1, 1), 0);
    }

    #[test]
    fn simple_path() {
        assert_eq!(dijkstra(vec![vec![1, 2, 1], vec![2, 3, 2]], 3, 1), 3);
    }

    #[test]
    fn unreachable() {
        assert_eq!(dijkstra(vec![vec![1, 2, 1], vec![2, 3, 1]], 4, 1), -1);
        assert_eq!(dijkstra(vec![vec![2, 1, 5]], 2, 1), -1);
    }

    #[test]
    fn multiple_paths() {
        let times = vec![vec![1, 2, 10], vec![1, 3, 2], vec![3, 2, 2]];
        assert_eq!(dijkstra(times, 3, 1), 4);
    }

    #[test]
    fn graph_with_cycle() {
        let times = vec![vec![1, 2, 1], vec![2, 3, 2], vec![3, 2, 1], vec![3, 4, 1]];
        assert_eq!(dijkstra(times, 4, 1), 4);
    }

    #[test]
    fn test_self_loop() {
        assert_eq!(dijkstra(vec![vec![1, 1, 5], vec![1, 2, 3],], 2, 1), 3);
    }

    #[test]
    fn zero_weight() {
        assert_eq!(dijkstra(vec![vec![1, 2, 0], vec![2, 3, 5]], 3, 1), 5);
    }

    #[test]
    fn weights_near_i32_max() {
        let times = vec![vec![1, 2, i32::MAX - 1], vec![2, 3, 10], vec![1, 3, 100]];
        assert_eq!(dijkstra(times, 3, 1), i32::MAX - 1);
    }

    #[test]
    fn start_with_last_node() {
        assert_eq!(dijkstra(vec![vec![1, 2, 1], vec![2, 3, 1]], 3, 3), -1);
    }
}
