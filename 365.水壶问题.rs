/*
 * @lc app=leetcode.cn id=365 lang=rust
 *
 * [365] 水壶问题
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        if z < 0 || z > x + y {
            return false;
        }
        if z == x || z == y {
            return true;
        }

        let permution: Vec<i32> = vec![x, y, -x, -y];

        let mut target_stack: Vec<i32> = vec![0]; // 目标栈
        let mut target_set: HashSet<i32> = HashSet::new();
        // target_set.insert(0);

        while let Some(target) = target_stack.pop() {
            // println!(target);
            // dbg!(target);
            permution.iter().for_each(|&mutation| {
                if ((mutation > 0 && target + mutation <= x + y)
                    || (mutation < 0 && target + mutation >= 0))
                    && target_set.insert(target + mutation)
                {
                    target_stack.push(target + mutation);
                }
            });
            // dbg!(&target_set);
            if target_set.contains(&z) {
                return true;
            }
        }

        false
    }
}
// @lc code=end
