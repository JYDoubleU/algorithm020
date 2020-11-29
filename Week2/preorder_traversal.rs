use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn traver(root: &Option<Rc<RefCell<TreeNode>>>, visited: &mut Vec<i32>) {
        if let Some(root) = root {
            let root = root.borrow();
            visited.push(root.val);
            traver(&root.left, visited);
            traver(&root.right, visited);
        }
    }

    let mut ret = vec![];
    if let Some(_) = root {
        traver(&root, &mut ret);
    }
    ret
}
