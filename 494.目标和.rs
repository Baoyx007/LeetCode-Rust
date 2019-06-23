use std::convert::TryInto;

pub fn add(u: usize, i: i32) -> usize {
  if i.is_negative() {
    u - i.wrapping_abs() as u32 as usize
  } else {
    u + i as usize
  }
}


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
    // value 是存在的个数
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
