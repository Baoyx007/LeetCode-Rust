/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */

// @lc code=start
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
// @lc code=end
