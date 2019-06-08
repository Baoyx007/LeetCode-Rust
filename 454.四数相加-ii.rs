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
