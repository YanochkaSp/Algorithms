#![allow(unused)]

use anyhow::{Result, anyhow};
use std::collections::HashSet;

#[derive(Default, Clone)]
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>, // вместо Box можно использовать Rc или Index (Index - лучше)
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn push_left_child(&mut self, child: TreeNode<T>) -> &mut Self {
        self.left = Some(Box::new(child));
        self
    }

    fn push_right_child(&mut self, child: TreeNode<T>) -> &mut Self {
        self.right = Some(Box::new(child));
        self
    }

    fn dfs_recursive(node: Option<Box<TreeNode<T>>>, res: &mut Vec<T>) {
        if let Some(n) = node {
            Self::dfs_recursive(n.left, res);
            res.push(n.val);
            Self::dfs_recursive(n.right, res);
        }
    }

    fn dfs_iterative(node: Option<Box<TreeNode<T>>>) -> Vec<T> {
        let mut res: Vec<T> = Vec::new();
        let mut queue = Vec::new();
        let mut cur = node;

        while cur.is_some() || !queue.is_empty() {
            while let Some(mut n) = cur {
                cur = n.left.take();
                queue.push(n);
            }
            if let Some(mut n) = queue.pop() {
                res.push(n.val);
                cur = n.right.take();
            }
        }
        res
    }
}

impl<T: Clone> TreeNode<T> {
    fn dfs_inorder_iterative(root: Option<&TreeNode<T>>) -> Vec<T> {
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
                res.push(node.val.clone());
                current = node.right.as_deref();
            }
        }
        res
    }
}

struct Tree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T> Tree<T> {
    fn new() -> Self {
        Self { root: None }
    }
}

impl<T: Clone> Tree<T> {
    fn inorder(&self) -> Vec<T> {
        TreeNode::dfs_inorder_iterative(self.root.as_deref())
    }

    fn build_tree_recursive(vals: &[Option<T>], index: usize) -> Option<Box<TreeNode<T>>> {
        if index >= vals.len() || vals[index].is_none() {
            return None;
        }

        Some(Box::new(TreeNode {
            val: vals[index].clone().expect("Expected value"),
            left: Self::build_tree_recursive(vals, 2 * index + 1),
            right: Self::build_tree_recursive(vals, 2 * index + 2),
        }))
    }

    // в массиве элементы лежат в порядке обходя в ширину
    fn build_tree_iterative(vals: &[Option<T>]) -> Option<Box<TreeNode<T>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let mut nodes: Vec<Option<TreeNode<T>>> = vals
            .iter()
            .map(|val| {
                val.clone().map(|v| TreeNode {
                    val: v,
                    left: None,
                    right: None,
                })
            })
            .collect();

        for i in (0..nodes.len()).rev() {
            if let Some(mut node) = nodes[i].take() {
                let left_idx = 2 * i + 1;
                let right_idx = 2 * i + 2;

                if left_idx < nodes.len() {
                    node.left = nodes[left_idx].take().map(Box::new);
                }
                if right_idx < nodes.len() {
                    node.right = nodes[right_idx].take().map(Box::new);
                }

                nodes[i] = Some(node);
            }
        }

        nodes[0].take().map(Box::new)
    }

    // push_left_child(), push_right_child() для build_tree_recursive
    fn build_tree_with_cursor(vals: &[Option<T>]) -> Option<Box<TreeNode<T>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let mut nodes: Vec<Option<TreeNode<T>>> = vals
            .iter()
            .map(|v| {
                v.clone().map(|x| TreeNode {
                    val: x,
                    left: None,
                    right: None,
                })
            })
            .collect();

        for i in (0..nodes.len()).rev() {
            if let Some(mut node) = nodes[i].take() {
                let left_idx = 2 * i + 1;
                let right_idx = 2 * i + 2;

                if left_idx < nodes.len() {
                    if let Some(left_child) = nodes[left_idx].take() {
                        node.push_left_child(left_child);
                    }
                }
                if right_idx < nodes.len() {
                    if let Some(right_child) = nodes[right_idx].take() {
                        node.push_right_child(right_child);
                    }
                }

                nodes[i] = Some(node);
            }
        }

        nodes[0].take().map(Box::new)
    }

    fn build_tree(vals: &[Option<T>], index: usize) -> Result<Box<TreeNode<T>>> {
        if index >= vals.len() || vals[index].is_none() {
            return Err(anyhow!("Root can't be None"));
        }

        Self::build_tree_with_cursor(vals).ok_or_else(|| anyhow!("Can't build tree"))
    }

    fn from_slice(vals: &[Option<T>]) -> Self {
        Self {
            root: Self::build_tree(vals, 0).ok(),
        }
    }
}

impl<T> Drop for Tree<T> {
    fn drop(&mut self) {
        let mut queue = Vec::new();
        queue.extend(self.root.take());
        while let Some(mut node) = queue.pop() {
            queue.extend(node.left.take());
            queue.extend(node.right.take());
        }
    }
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
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(8),
            None,
            None,
            Some(6),
            Some(7),
            None,
            None,
            Some(9),
            None,
        ]);
        assert_eq!(root.inorder(), [4, 2, 6, 5, 7, 1, 3, 9, 8]);
    }

    #[test]
    fn empty() {
        let root: Tree<i8> = Tree::from_slice(&vec![]);
        assert_eq!(root.inorder(), []);
    }

    #[test]
    fn single() {
        let root = Tree::from_slice(&vec![Some(1)]);
        assert_eq!(root.inorder(), [1]);
    }

    fn create_tree() -> Vec<Option<i8>> {
        vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]
    }

    #[test]
    fn all_implementations_agree() {
        let vals = create_tree();

        let tree1 = Tree::from_slice(&vals);
        let inorder_res = tree1.inorder();

        let tree2 = Tree::from_slice(&vals);
        let root_for_iterative = Tree::<i8>::build_tree_iterative(&vals);
        let iterative_res = TreeNode::dfs_iterative(root_for_iterative);

        let tree3 = Tree::from_slice(&vals);
        let root_for_recursive = Tree::<i8>::build_tree_iterative(&vals);
        let mut recursive_res = Vec::new();
        TreeNode::dfs_recursive(root_for_recursive, &mut recursive_res);

        assert_eq!(inorder_res, recursive_res);
        assert_eq!(inorder_res, iterative_res);
        assert_eq!(recursive_res, iterative_res);
    }

    #[test]
    fn left_skewed_tree() {
        let vals = vec![
            Some(1),
            Some(2),
            None,
            Some(3),
            None,
            None,
            None,
            Some(4),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let tree = Tree::from_slice(&vals);
        let inorder = tree.inorder();
        assert_eq!(inorder, vec![4, 3, 2, 1]);
    }

    #[test]
    fn each_node_visited_exactly_once() {
        let vals = vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ];
        let tree = Tree::from_slice(&vals);
        let inorder = tree.inorder();

        assert_eq!(inorder.len(), 7);
        let mut unique_vals: HashSet<i8> = inorder.iter().cloned().collect();
        assert_eq!(unique_vals.len(), 7);
    }
}
