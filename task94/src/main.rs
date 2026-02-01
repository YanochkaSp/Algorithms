#![allow(unused)]

struct TreeNode {
    val: i8,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
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
        if let Some(root) = self.root.take() {
            queue.push(root);
        }
        while let Some(mut node) = queue.pop() {
            if let Some(left) = node.left.take() {
                queue.push(left);
            }
            if let Some(right) = node.right.take() {
                queue.push(right);
            }
        }
    }
}

fn dfs_recursive(node: Option<Box<TreeNode>>, res: &mut Vec<i8>) {
    if let Some(n) = node {
        dfs_recursive(n.left, res);
        res.push(n.val);
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
            res.push(node.val);
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
            res.push(n.val);
            cur = n.right.take();
        }
    }
    res
}

fn build_tree_recursive(vals: &[Option<i8>], index: usize) -> Option<Box<TreeNode>> {
    if index >= vals.len() || vals[index].is_none() {
        return None;
    }

    Some(Box::new(TreeNode {
        val: vals[index].expect("Expected value"),
        left: build_tree_recursive(vals, 2 * index + 1),
        right: build_tree_recursive(vals, 2 * index + 2),
    }))
}

fn build_tree_iterative(vals: &[Option<i8>]) -> Option<Box<TreeNode>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }

    let mut nodes: Vec<Option<Box<TreeNode>>> = vals.iter().map(|&val| val.map(|v| Box::new(TreeNode{ val: v, left: None, right: None }))).collect();

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

#[test]
fn simple_test() {
    let root = Tree::from_slice(&[Some(1), None, Some(2), None, None, Some(3)]);
    assert_eq!(root.inorder(), [1, 3, 2]);
}

#[test]
fn complex_test() {
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
fn empty_test() {
    let root = Tree::from_slice(&vec![]);
    assert_eq!(root.inorder(), []);
}

#[test]
fn single_test() {
    let root = Tree::from_slice(&vec![Some(1)]);
    assert_eq!(root.inorder(), [1]);
}
