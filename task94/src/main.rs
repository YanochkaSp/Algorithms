struct TreeNode {
    val: u8,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}
fn main() {}

fn dfs(node: Option<Box<TreeNode>>, res: &mut Vec<u8>) {
    if let Some(n) = node {
        dfs(n.left, res);
        res.push(n.val);
        dfs(n.right, res);
    }
}

fn build_tree_recursive(vals: &[Option<u8>], index: usize) -> Option<Box<TreeNode>> {
    if index >= vals.len() || vals[index].is_none() {
        return None;
    }

    Some(Box::new(TreeNode {
        val: vals[index].unwrap(),
        left: build_tree_recursive(vals, 2 * index + 1),
        right: build_tree_recursive(vals, 2 * index + 2),
    }))
}

#[test]
fn simple_test() {
    let mut res = Vec::new();
    let root = build_tree_recursive(&vec![Some(1), None, Some(2), None, None, Some(3), None], 0);
    dfs(root, &mut res);
    assert_eq!(res, [1, 3, 2]);
}
