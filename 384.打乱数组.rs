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