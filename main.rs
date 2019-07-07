struct Solution;

/*
 * @lc app=leetcode.cn id=876 lang=rust
 *
 * [876] 链表的中间结点
 */
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>,
// }

// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
impl Solution {
  pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;

    while fast.is_some() && fast.unwrap().next.is_some()  {
      
      slow = &slow.unwrap().next;
      fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    *slow.clone().as_ref()
  }
}

fn main() {
  // println!("{:?}", Solution::middle_node(Some(list![1,2,3,4,5])));
}
