/*
*
* dp[n][m] =  min(dp[n][m-1] + insert(b) , dp[n-1][m] + insert(a), dp[n-1][m-1] + switch(a->b) )
*          = min(dp[n][m-1] +1,  dp[n-1][m] +1,  dp[n-1][m-1] + 1 )
*
* @lc app=leetcode.cn id=72 lang=rust
*
* [72] 编辑距离
*/

// @lc code=start
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // Vec::
        let mut word1 = word1.clone();
        let mut word2 = word2.clone();
        word1.insert(0, '#');
        word2.insert(0, '#');
        let word1_len = word1.len();
        let word2_len = word2.len();

        // let mut dp =vec![[0;word2_len];word1_len];
        let mut dp: Vec<Vec<i32>> = Vec::new();

        // dbg!(dp);
        for (i, x) in word1.chars().enumerate() {
            dp.push(vec![0; word2_len]);
            for (j, y) in word2.chars().enumerate() {
                if i == 0 {
                    dp[i][j] = j as i32;
                } else if j == 0 {
                    dp[i][j] = i as i32;
                } else if x != y {
                    dp[i][j] = 1 + min!(dp[i][j - 1], dp[i - 1][j], dp[i - 1][j - 1]);
                } else {
                    dp[i][j] = min!(dp[i][j - 1] + 1, dp[i - 1][j] + 1, dp[i - 1][j - 1]);
                }
            }
        }

        dp[word1_len - 1][word2_len - 1]
    }
}
