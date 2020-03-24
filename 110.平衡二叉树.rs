/*
 * @lc app=leetcode.cn id=110 lang=rust
 *
 * [110] 平衡二叉树
 */

// @lc code=start
// Definition for a binary tree node.
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        false
    }

    fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => -1,
            Some(tree) => 1 + Solution::height(tree.borrow().left.as_ref()),
        }
    }
}

// @lc code=end
