impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
      coins.iter().fold(0, | pre, c| pre + (c/2+c%2) )

    }
}
