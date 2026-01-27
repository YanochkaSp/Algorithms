struct TreeNode {
    val: u8,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}
fn main() {
    let mut res = Vec::new();
    fn dfs(node: Option<Box<TreeNode>>, res: &mut Vec<u8>) {
        if let Some(n) = node {
            dfs(n.left, res);
            res.push(n.val);
            dfs(n.right, res);
        }
    }
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            })),
            right: None,
        })),
    }));
    dfs(root, &mut res);
    println!("{:?}", res);
}

fn build_tree() {
    
}