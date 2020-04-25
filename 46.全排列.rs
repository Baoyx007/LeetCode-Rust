/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] 全排列
 */

// @lc code=start
impl Solution {
    fn bt(nums: &Vec<i32>, ret: &mut Vec<Vec<i32>>, track: &mut Vec<i32>) {
        if track.len() == nums.len() {
            ret.push(track.clone());
            return;
        }
        for n in nums {
            if track.contains(&n) {
                continue;
            }
            track.push(*n);
            Self::bt(nums, ret, track);
            track.pop();
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();

        Self::bt(&nums, &mut ret, &mut vec![]);
        ret
    }
}
// @lc code=end
