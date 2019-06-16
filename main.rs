struct Solution;

use leetcode::vec2d;
/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] 不同路径
 */
impl Solution {
  pub fn unique_paths(m: i32, n: i32) -> i32 {
    if m <= 0 || n <= 0 {
      return 0;
    }
    if m == 1 || n == 1 {
      return 1;
    }
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![0; n]; m];
    dp[0][1] = 1;
    dp[1][0] = 1;

    for i in 0..m {
      for j in 0..n {
        dp[i][j] = match (i.checked_sub(1), j.checked_sub(1)) {
          (Some(pre_i), Some(pre_j)) => dp[pre_i][j] + dp[i][pre_j],
          (None, None) => 0,
          _ => 1,
        };
        // println!("{} {} , {}", i, j, dp[i][j]);
      }
    }
    dp[m-1][n-1]
  }
}

fn main() {
  // println!("{:?}", vec2d[vec![]])
  println!("{}", Solution::unique_paths(7, 3));
}
