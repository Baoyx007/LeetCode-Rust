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