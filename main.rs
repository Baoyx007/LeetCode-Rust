struct Solution;

/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * 23. 合并K个排序链表
 */

// Definition for singly-linked list.
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
// @lc code=start

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut ret_head = Box::new(ListNode::new(-1));

        loop {
            let mut min = -1;
            let mut min_ptr: &Option<Box<ListNode>> = &None;
            for l in &lists {
                if let Some(node) = l {
                    if node.val < min {
                        min = node.val;
                        min_ptr = l;
                    }
                }
            }

            if let Some(node) = min_ptr {
                ret_head.next = Some(node.clone());
                ret_head = ret_head.next.unwrap();

                min_ptr = node.next;
            }
        }

        None
    }
}
// @lc code=end

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
