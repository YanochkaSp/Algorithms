#![allow(unused)]

use anyhow::{anyhow, Result};
use std::collections::HashSet;

#[derive(Default)]
struct TreeNode {
    val: Box<i8>,
    left: Option<Box<TreeNode>>, //TODO Box можно использовать Rc или Index (Index - лучше)
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode::default()
    }

    fn push_left_child(&mut self, child: TreeNode) -> &mut Self {
        self.left = Some(Box::new(child));
        self
    }

    fn push_right_child(&mut self, child: TreeNode) -> &mut Self {
        self.right = Some(Box::new(child));
        self
    }
}

struct Tree {
    root: Option<Box<TreeNode>>,
}

impl Tree {
    fn new() -> Self {
        Self { root: None }
    }

    fn from_slice(vals: &[Option<i8>]) -> Self {
        Self {
            root: build_tree_iterative(vals),
        }
    }

    fn inorder(&self) -> Vec<i8> {
        dfs_inorder_iterative(self.root.as_deref())
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        let mut queue = Vec::new();
        queue.extend(self.root.take());
        while let Some(mut node) = queue.pop() {
            queue.extend(node.left.take());
            queue.extend(node.right.take());
        }
    }
}

fn dfs_recursive(node: Option<Box<TreeNode>>, res: &mut Vec<i8>) {
    if let Some(n) = node {
        dfs_recursive(n.left, res);
        res.push(*n.val);
        dfs_recursive(n.right, res);
    }
}

fn dfs_inorder_iterative(root: Option<&TreeNode>) -> Vec<i8> {
    // без take()
    let mut res = Vec::new();
    let mut stack = Vec::new();
    let mut current = root;

    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            stack.push(node);
            current = node.left.as_deref();
        }
        if let Some(node) = stack.pop() {
            res.push(*node.val);
            current = node.right.as_deref();
        }
    }
    res
}

fn dfs_iterative(node: Option<Box<TreeNode>>) -> Vec<i8> {
    let mut res: Vec<i8> = Vec::new();
    let mut queue = Vec::new();
    let mut cur = node;

    while cur.is_some() || !queue.is_empty() {
        while let Some(mut n) = cur {
            cur = n.left.take();
            queue.push(n);
        }
        if let Some(mut n) = queue.pop() {
            res.push(*n.val);
            cur = n.right.take();
        }
    }
    res
}

fn build_tree(vals: &[Option<i8>], index: usize) -> Result<Box<TreeNode>> {
    if index >= vals.len() || vals[index].is_none() {
        return Err(anyhow!("Root can't be None"));
    }

    build_tree_with_cursor(vals).ok_or_else(|| anyhow!("Can't build tree"))
}

fn build_tree_recursive(vals: &[Option<i8>], index: usize) -> Option<Box<TreeNode>> {
    if index >= vals.len() || vals[index].is_none() {
        return None;
    }

    Some(Box::new(TreeNode {
        val: Box::new(vals[index].expect("Expected value")),
        left: build_tree_recursive(vals, 2 * index + 1),
        right: build_tree_recursive(vals, 2 * index + 2),
    }))
}

// push_left_child(), push_right_child() для build_tree_recursive
fn build_tree_with_cursor(vals: &[Option<i8>]) -> Option<Box<TreeNode>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }

    let mut nodes: Vec<Option<Box<TreeNode>>> = vals
        .iter()
        .map(|&v| {
            v.map(|x| {
                Box::new(TreeNode {
                    val: Box::new(x),
                    left: None,
                    right: None,
                })
            })
        })
        .collect();

    let mut parent_index = 0;
    let mut child_index = 1;

    while child_index < nodes.len() {
        while parent_index < nodes.len() && nodes[parent_index].is_none() {
            parent_index += 1;
        }
        if parent_index >= nodes.len() {
            break;
        }

        if child_index < nodes.len() {
            if let Some(child) = nodes[child_index].take() {
                nodes[parent_index].as_mut().expect("Parent node must exist").push_left_child(*child);
            }
            child_index += 1;
        }

        if child_index < nodes.len() {
            if let Some(child) = nodes[child_index].take() {
                nodes[parent_index].as_mut().expect("Parent node must exist").push_right_child(*child);
            }
            child_index += 1;
        }

        parent_index += 1;
    }

    nodes[0].take()
}

// в массиве элементы лежат в порядке обходя в ширину
fn build_tree_iterative(vals: &[Option<i8>]) -> Option<Box<TreeNode>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }

    let mut nodes: Vec<Option<Box<TreeNode>>> = vals
        .iter()
        .map(|&val| {
            val.map(|v| {
                Box::new(TreeNode {
                    val: Box::new(v),
                    left: None,
                    right: None,
                })
            })
        })
        .collect();

    for i in (0..nodes.len()).rev() {
        if let Some(mut node) = nodes[i].take() {
            let left_idx = 2 * i + 1;
            let right_idx = 2 * i + 2;

            if left_idx < nodes.len() {
                node.left = nodes[left_idx].take();
            }
            if right_idx < nodes.len() {
                node.right = nodes[right_idx].take();
            }

            nodes[i] = Some(node);
        }
    }

    nodes[0].take()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let root = Tree::from_slice(&[Some(1), None, Some(2), None, None, Some(3)]);
        assert_eq!(root.inorder(), [1, 3, 2]);
    }

    #[test]
    fn complex() {
        let root = Tree::from_slice(&vec![
            Some(1),
            Some(2), Some(3),
            Some(4), Some(5), None, Some(8), 
            None, None, Some(6), Some(7), None, None, Some(9), None,
        ]);
        assert_eq!(root.inorder(), [4, 2, 6, 5, 7, 1, 3, 9, 8]);
    }

    #[test]
    fn empty() {
        let root = Tree::from_slice(&vec![]);
        assert_eq!(root.inorder(), []);
    }

    #[test]
    fn single() {
        let root = Tree::from_slice(&vec![Some(1)]);
        assert_eq!(root.inorder(), [1]);
    }

    fn create_tree() -> Vec<Option<i8>> {
        vec![ Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]
    }

    #[test]
    fn all_implementations_agree() {
        let vals = create_tree();

        let tree1 = Tree::from_slice(&vals);
        let inorder_res = tree1.inorder();

        let tree2 = Tree::from_slice(&vals);
        let root_for_iterative = build_tree_iterative(&vals);
        let iterative_res = dfs_iterative(root_for_iterative);

        let tree3 = Tree::from_slice(&vals);
        let root_for_recursive = build_tree_iterative(&vals);
        let mut recursive_res = Vec::new();
        dfs_recursive(root_for_recursive, &mut recursive_res);

        assert_eq!(inorder_res, recursive_res);
        assert_eq!(inorder_res, iterative_res);
        assert_eq!(recursive_res, iterative_res);
    }

    #[test]
    fn left_skewed_tree() {
        let vals = vec![
            Some(1),
            Some(2), None,
            Some(3), None, None, None,
            Some(4), None, None, None, None, None, None, None,
        ];
        let tree = Tree::from_slice(&vals);
        let inorder = tree.inorder();
        assert_eq!(inorder, vec![4, 3, 2, 1]);
    }

    #[test]
    fn each_node_visited_exactly_once() {
        let vals = vec![
            Some(1),
            Some(2), Some(3),
            Some(4), Some(5), Some(6), Some(7)
        ];
        let tree = Tree::from_slice(&vals);
        let inorder = tree.inorder();

        assert_eq!(inorder.len(), 7);
        let mut unique_vals: HashSet<i8> = inorder.iter().cloned().collect();
        assert_eq!(unique_vals.len(), 7);
    }
}
