struct TreeNode {
    val: i8,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}
fn main() {}

fn dfs(node: Option<Box<TreeNode>>, res: &mut Vec<i8>) {
    if let Some(n) = node {
        dfs(n.left, res);
        res.push(n.val);
        dfs(n.right, res);
    }
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

#[test]
fn simple_test() {
    let mut res = Vec::new();
    let root = build_tree_recursive(&vec![Some(1), None, Some(2), None, None, Some(3), None], 0);
    dfs(root, &mut res);
    assert_eq!(res, [1, 3, 2]);
}

#[test]
fn complex_test() {
    let mut res = Vec::new();
    let root = build_tree_recursive(&vec![Some(1), Some(2), Some(3), Some(4), Some(5), None,  Some(8), None, None, Some(6), Some(7), None, None, Some(9), None], 0);
    dfs(root, &mut res);
    assert_eq!(res, [4,2,6,5,7,1,3,9,8]);
}

#[test]
fn empty_test() {
    let mut res = Vec::new();
    let root = build_tree_recursive(&vec![], 0);
    dfs(root, &mut res);
    assert_eq!(res, []);
}

#[test]
fn single_test() {
    let mut res = Vec::new();
    let root = build_tree_recursive(&vec![Some(1)], 0);
    dfs(root, &mut res);
    assert_eq!(res, [1]);
}
