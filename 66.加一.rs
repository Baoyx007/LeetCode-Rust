/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut sum = 1;
        let mut ret: Vec<i32> = Vec::new();

        for d in digits.iter().rev() {
            sum += d;

            if sum >= 10 {
                ret.insert(0, sum - 10);
                sum = 1;
            } else {
                ret.insert(0, sum);
                sum = 0;
            }
        }

        if sum > 0 {
            ret.insert(0, sum)
        }

        ret
    }
}
// @lc code=end
