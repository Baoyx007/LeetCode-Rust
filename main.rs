use leetcode::ListNode;

struct Solution;
/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 */
// Definition for singly-linked list.

impl Solution {
  pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(mut hp) = head {
      // foo.
      let recv = Solution::reverse_list(hp.next);
      if let Some(p) = recv {
              hp.next.next = hp;
      } else {
        // head
      }
    }else{
 None
    }
   
  }
}

fn main() {
  // println!("{:?}", Solution::reverse_list());
}
