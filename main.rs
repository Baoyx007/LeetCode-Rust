struct Solution;

macro_rules! vec2d {
    ($($arr:tt),*) => { // handle sets
        {
            let mut ret= Vec::new();
            $(ret.push(vec!$arr);)*
            ret
        }
    };
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Item(i32, usize, usize);

impl Ord for Item {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0).reverse()
  }
}

impl PartialOrd for Item {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Solution {
  // pub fn mid_find(matrix: &Vec<Vec<i32>>)

  pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let len = matrix.len();
    if len == 0 {
      return 0;
    };
    let row_len = matrix[0].len();

    let mut heap = BinaryHeap::with_capacity(k as usize);

    for i in 0..len {
      heap.push(Item(matrix[i][0], i, 0));
    }

    let mut min = Item(matrix[0][0], 0, 0);
    for _ in 0..k {
      min = heap.pop().unwrap();
      if min.2 + 1 < row_len {
        heap.push(Item(matrix[min.1][min.2 + 1], min.1, min.2 + 1));
      }
    }

    min.0
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
  let obj = Solution::kth_smallest(vec2d![[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8);
  // println!("{:?}", Solution::search_range(vec![1, 4], 4));
}
