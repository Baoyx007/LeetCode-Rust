use rand::seq::SliceRandom;
use rand::thread_rng;

/*
 * @lc app=leetcode.cn id=384 lang=rust
 *
 * [384] 打乱数组
 */
struct Solution {
  nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
  fn new(nums: Vec<i32>) -> Self {
    Solution { nums }
  }

  /** Resets the array to its original configuration and return it. */
  fn reset(&self) -> Vec<i32> {
    self.nums.clone()
  }

  /** Returns a random shuffling of the array. */
  fn shuffle(&self) -> Vec<i32> {
    let mut ret = self.nums.clone();
    let mut rng = thread_rng();
    ret.shuffle(&mut rng);
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
  let obj = Solution::new(vec![1, 2, 3]);
  let ret_1: Vec<i32> = obj.reset();
  println!("{:?}", ret_1);
  let ret_2: Vec<i32> = obj.shuffle();
  println!("{:?}", ret_2);
  // println!("{:?}", Solution::search_range(vec![1, 4], 4));
}
