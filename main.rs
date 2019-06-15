struct Solution;

/*
 * @lc app=leetcode.cn id=139 lang=rust
 *
 * [139] 单词拆分
 */
impl Solution {
  pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut dp: Vec<bool> = Vec::with_capacity(s.len() + 1);
    dp.push(true);

    for i in 1..=s.len() {
      dp.push(false);
      for j in 0..i {
        // println!("{} {}", dp[j], j);
        if dp[j] && word_dict.contains(&s[j..i].to_string()) {
          dp[i] = true;
          break;
        }
      }
    }
    println!("{:?}", dp);
    dp[s.len()]
  }
}

fn main() {
  println!(
    "{}",
    Solution::word_break(
      "applepenapple".to_string(),
      vec!["apple".to_string(), "pen".to_string()]
    )
  );
}
