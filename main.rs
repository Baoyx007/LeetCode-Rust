use std::collections::HashMap;

struct Solution;

/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * 23. 合并K个排序链表
 */

// Definition for singly-linked list.
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut ret = vec![0; n + 1];

        if n <= 2 {
            return n as i32;
        }
        ret[1] = 1;
        ret[2] = 2;

        for i in 3..=n {
            ret[i] = ret[i - 1] + ret[i - 2]
        }

        // dbg!(&ret);
        *ret.get(n).unwrap() as i32
    }
}

fn main() {
    print!("{}", Solution::climb_stairs(1))
}
