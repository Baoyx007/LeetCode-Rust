struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

impl std::fmt::Display for ListNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.next {
      None => write!(f, "{}", self.val),
      Some(next) => write!(f, "{} -> {}", self.val, next),
    }
  }
}

// macro_rules! linked {

//     ($($arr:tt),*) => { // handle sets
//         {
//             let mut ret= Vec::new();
//             $(ret.push(Box::new(ListNode::new($arr)));)*
//             ret.
//             ret
//         }
//     };
// }

use core::cmp::Ordering;
use std::collections::BinaryHeap;

impl Ord for ListNode {
  fn cmp(&self, other: &Self) -> Ordering {
    self.val.cmp(&other.val).reverse()
  }
}

impl PartialOrd for ListNode {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Solution {
  pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let head = Box::new(ListNode::new(-1));
    let mut priority_queue = BinaryHeap::with_capacity(lists.len());

    for vec in lists {
      if let Some(node) = vec {
        priority_queue.push(node);
      }
    }

    println!("{:?}", &priority_queue);

    let mut tmp = &head;
    while let Some(node) = priority_queue.pop() {
      println!("{:?}", &head.next);
      tmp.next = Some(node);
      tmp = &tmp.next.unwrap();
      if let Some(next) = tmp.next {
        priority_queue.push(next);
        tmp.next = None;
      }
    }

    head.next
  }
}

#[test]
fn case() {
  assert_eq!(
    Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
    vec![3, 4]
  );
  assert_eq!(
    Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
    vec![-1, -1]
  );
  assert_eq!(Solution::search_range(vec![1], 0), vec![-1, -1]);
}

fn main() {
  let h1 = {
    let mut head = Box::new(ListNode::new(1));
    let mut l2 = Box::new(ListNode::new(4));
    l2.next = Some(Box::new(ListNode::new(5)));
    head.next = Some(l2);
    Some(head)
  };

  let h2 = {
    let mut head = Box::new(ListNode::new(1));
    let mut l2 = Box::new(ListNode::new(3));
    l2.next = Some(Box::new(ListNode::new(4)));
    head.next = Some(l2);
    Some(head)
  };

  let h3 = {
    let mut head = Box::new(ListNode::new(2));
    let mut l2 = Box::new(ListNode::new(6));
    head.next = Some(l2);
    Some(head)
  };

  let ret = Solution::merge_k_lists(vec![h1, h2, h3]);
  println!("{:?}", ret);
}
