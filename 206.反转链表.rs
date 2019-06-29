/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 */
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
  pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
      Some(mut p) => {
        let rev = Solution::reverse_list(p.clone().next); 
        // println!("{:?}", p); 
        // println!("{:?}", rev);
        // rev.next = p.next
        p.next = None;

        match rev {
          Some(mut pp) => {
            pp.next = Some(p);
            Some(pp)
          }
          None => Some(p),
        }
      }
      None => None,
    }
  }
}
