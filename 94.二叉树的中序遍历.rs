/*
 * @lc app=leetcode.cn id=94 lang=rust
 *
 * [94] 二叉树的中序遍历
 */
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
   pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
    let mut ret: Vec<i32> = vec![];

    let mut ptr = root;
    while ptr.is_some() || !stack.is_empty() {
      while let Some(p) = ptr {
        stack.push(p.clone());
        ptr = p.borrow().left.clone();
      }

      let top = stack.pop().unwrap();
      ret.push(top.borrow().val);

      ptr = top.borrow().right.clone();
    }

    ret
  }
}
