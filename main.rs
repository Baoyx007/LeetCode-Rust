use leetcode::ListNode;
use std::convert::TryInto;

struct Solution;
/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 */
// Definition for singly-linked list.

impl Solution {
  pub fn generate_parenthesis(n: i32) -> Vec<String> {
    if n <= 0 {
      return vec![];
    }
    let n = n as usize;
    let mut ret = vec![];

    Solution::backtrack(&mut ret, "".to_owned(), 0, 0, n * 2);

    ret
  }

  fn backtrack(ret: &mut Vec<String>, cur: String, open: usize, close: usize, max: usize) {
    if cur.len() == max {
      ret.push(cur);
      return;
    }

    if open < (max / 2) {
      Solution::backtrack(ret, cur.clone() + "(", open + 1, close, max)
    }

    if close < open {
      Solution::backtrack(ret, cur + ")", open, close + 1, max)
    }
  }
}

fn main() {
  println!("{:?}", Solution::generate_parenthesis(3));
}
