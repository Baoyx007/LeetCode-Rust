struct Solution;
/*
 * @lc app=leetcode.cn id=621 lang=rust
 *
 * [621] 任务调度器
 */
impl Solution {
  pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut bucket = [0; 26];
    let mut max = 0;
    for &c in &tasks {
      let ascii = c as usize - 65;
      bucket[ascii] += 1;
      if bucket[ascii] > max {
        max = bucket[ascii];
      }
    }

    let mut res = (max - 1) * (n + 1);

    for &count in bucket.iter() {
      if count == max {
        res += 1;
      }
    }

    let len = tasks.len() as i32;
    if res > len {
      res
    } else {
      len
    }
  }
}

fn main() {
  // println!('{:?}', vec2d[vec![]])
  println!(
    "{}",
    Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2)
  );
}
