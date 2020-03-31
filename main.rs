struct Solution;
// @lc code=start
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = nums.clone();
        ret.sort();
        ret
    }
}
// @lc code=end

fn main() {
    println!("{:?}", Solution::sort_array(vec![5, 2, 3, 1]));
}
