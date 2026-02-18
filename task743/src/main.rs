#![allow(unused)]
use thiserror::Error;

#[derive(Clone, Copy)]
struct NodeId(usize);

impl NodeId {
    fn new(id: usize, max: usize) -> Result<Self, GraphError> {
        if id >= max {
            Err(GraphError::InvalidNode {
                id,
                max,
            })
        } else {
            Ok(Self(id))
        }
    }

    fn to_zero_based_numbering(id: usize, max: usize) -> Result<Self, GraphError> {
        if id == 0 || id > max {
            Err(GraphError::InvalidNode { id, max })
        } else {
            Ok(Self(id - 1))
        }
    }
}

#[derive(Clone, Copy)]
struct Weight(i32);

impl Weight {
    fn new(weight: i32) -> Result<Self, GraphError> {
        if weight < 0 {
            Err(GraphError::NegativeWeight(weight))
        } else {
            Ok(Self(weight))
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GraphError {
    #[error("Graph has no nodes")]
    Empty,

    #[error("Node {id} out of range {max}")]
    InvalidNode { id: usize, max: usize },

    #[error("Some nodes are unreachable from start")]
    Unreachable,

    #[error("Edge has negative weight: {0}")]
    NegativeWeight (i32),
}

struct Graph {
    adjacency: Vec<Vec<(NodeId, Weight)>>,
    n: usize,
}

impl Graph {
    fn new(n: usize) -> Result<Self, GraphError> {
        if n == 0 {
            return Err(GraphError::Empty);
        }
        Ok(Self {
            adjacency: vec![Vec::new(); n],
            n,
        })
    }

    fn add_edge(&mut self, from: NodeId, to: NodeId, weight: Weight) -> Result<(), GraphError> {
        self.adjacency[from.0 as usize].push((to, weight));
        Ok(())
    }

    fn build_graph(n: usize, edges: Vec<[usize; 3]>) -> Result<Self, GraphError> {
        let mut graph = Self::new(n)?;
        for [from, to, weight] in edges {
            let from_id = NodeId::to_zero_based_numbering(from, n)?;
            let to_id = NodeId::to_zero_based_numbering(to, n)?;
            let w = Weight::new(weight as i32)?;
            graph.add_edge(from_id, to_id, w)?;
        }
        Ok(graph)
    }

    fn delay_from(&self, start: usize) -> Result<i32, GraphError> {
        if start >= self.n {
            return Err(GraphError::InvalidNode {
                id: start,
                max: self.n,
            });
        }

        let mut dists = vec![i32::MAX; self.n];
        dists[start] = 0;

        let mut queue = Vec::new();
        queue.push((0, start));

        while let Some((dist, u)) = queue.pop() {
            if dist > dists[u] {
                continue;
            }
            for &(v, w) in &self.adjacency[u] {
                let weight = w.0 as i32;
                let v_id = v.0 as usize;
                if let Some(new_dist) = dists[u].checked_add(weight) {
                    if new_dist < dists[v_id] {
                        dists[v_id] = new_dist;
                        queue.push((dists[v_id], v_id));
                    }
                }
            }
        }

        let time = *(dists.iter().max().ok_or(GraphError::Empty)?);

        if time == i32::MAX { Err(GraphError::Unreachable) } else { Ok(time) }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single() -> Result<(), GraphError> {
        let g = Graph::new(1)?;
        assert_eq!(g.delay_from(0), Ok(0));
        Ok(())
    }

    #[test]
    fn simple_path() -> Result<(), GraphError> {
        let g = Graph::build_graph(3, vec![
            [1, 2, 1],
            [2, 3, 2],
        ])?;
        assert_eq!(g.delay_from(0), Ok(3));
        Ok(())
    }

    #[test]
    fn unreachable() -> Result<(), GraphError> {
        let g = Graph::build_graph(4, vec![
            [1, 2, 1],
            [2, 3, 1],
        ])?;
        assert_eq!(g.delay_from(0), Err(GraphError::Unreachable));

        let g2 = Graph::build_graph(2, vec![
            [2, 1, 5],
        ])?;
        assert_eq!(g2.delay_from(0), Err(GraphError::Unreachable));
        Ok(())
    }

    #[test]
    fn multiple_paths() -> Result<(), GraphError> {
        let g = Graph::build_graph(3, vec![
            [1, 2, 10],
            [1, 3, 2],
            [3, 2, 2],
        ])?;
        assert_eq!(g.delay_from(0), Ok(4));
        Ok(())
    }

    #[test]
    fn graph_with_cycle() -> Result<(), GraphError> {
        let g = Graph::build_graph(4, vec![
            [1, 2, 1],
            [2, 3, 2],
            [3, 2, 1],
            [3, 4, 1],
        ])?;
        assert_eq!(g.delay_from(0), Ok(4));
        Ok(())
    }

    #[test]
    fn self_loop() -> Result<(), GraphError> {
        let g = Graph::build_graph(2, vec![
            [1, 1, 5],
            [1, 2, 3],
        ])?;
        assert_eq!(g.delay_from(0), Ok(3));
        Ok(())
    }

    #[test]
    fn zero_weight() -> Result<(), GraphError> {
        let g = Graph::build_graph(3, vec![
            [1, 2, 0],
            [2, 3, 5],
        ])?;
        assert_eq!(g.delay_from(0), Ok(5));
        Ok(())
    }

    #[test]
    fn weights_near_i32_max() -> Result<(), GraphError> {
        let g = Graph::build_graph(3, vec![
            [1, 2, (i32::MAX - 1) as usize],
            [2, 3, 10],
            [1, 3, 100],
        ])?;
        assert_eq!(g.delay_from(0), Ok(i32::MAX - 1));
        Ok(())
    }

    #[test]
    fn start_with_last_node() -> Result<(), GraphError> {
        let g = Graph::build_graph(3, vec![
            [1, 2, 1],
            [2, 3, 1],
        ])?;
        assert_eq!(g.delay_from(2), Err(GraphError::Unreachable));
        Ok(())
    }
}
