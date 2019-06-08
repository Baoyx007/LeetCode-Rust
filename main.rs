struct Solution;

use std::collections::HashMap;

impl Solution {
  pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    let mut ab_map: HashMap<i32, i32> = HashMap::new();
    let mut ret = 0;

    for x in &a {
      for y in &b {
        match ab_map.get(&(x + y)) {
          None => ab_map.insert(x + y, 1),
          Some(&time) => ab_map.insert(x + y, time + 1),
        };
      }
    }

    for x in &c {
      for y in &d {
        if let Some(time) = ab_map.get(&(-x - y)) {
          ret += time;
        };
      }
    }

    ret
  }
}

#[test]
fn case() {
  assert_eq!(
    Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
    vec![3, 4]
  );
  assert_eq!(
    Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
    vec![-1, -1]
  );
  assert_eq!(Solution::search_range(vec![1], 0), vec![-1, -1]);
}

fn main() {
  let obj = Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]);
  // println!("{:?}", Solution::search_range(vec![1, 4], 4));
}
