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