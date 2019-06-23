struct Solution;

use leetcode::add;
use std::convert::TryInto;
// use leetcode::add
/*
 * @lc app=leetcode.cn id=494 lang=rust
 *
 * [494] 目标和
 */
impl Solution {
  pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
    // let nums:  Vec<usize>= nums.into();
    let max: usize = nums.iter().sum::<i32>().try_into().unwrap();
    if s > max.try_into().unwrap() || s < -(max as i32) {
      return 0;
    }
    // -max ... 0    ... max, dp 范围
    // 0    ... max  ... 2max
    // index 其实是算出的值
    let mut dp = vec![0; max * 2 + 1];
    dp[max] = 1; // 第一轮 dp, nums 一个都没选, 那么只有一种情况, 值为 0
    for num in nums {
      let mut new_dp = vec![0; max * 2 + 1];
      for i in 0..=(max * 2) {
        if dp[i] > 0 {
          new_dp[add(i, -num)] += dp[i];
          new_dp[add(i, num)] += dp[i];
        }
      }
      dp = new_dp;
      // println!("{:?}", &dp);
    }

    dp[add(max, s)]
  }
}

fn main() {
  // println!('{:?}', vec2d[vec![]])
  println!("{}", Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
}
