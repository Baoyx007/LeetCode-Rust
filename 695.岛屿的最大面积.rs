#![feature(trace_macros)]
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

use std::cmp;

/*
 * @lc app=leetcode.cn id=695 lang=rust
 *
 * [695] 岛屿的最大面积
 */
impl Solution {
  fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    println!("[{},{}]", x, y);
    if x >= grid.len() || y >= grid[0].len() {
      return 0;
    }

    if grid[x][y] == 0 {
      return 0;
    }

    grid[x][y] = 0; // mark visited

    1 + if x >= 1 {
      Solution::dfs(grid, x - 1, y)
    } else {
      0
    } + Solution::dfs(grid, x + 1, y)
      + if y >= 1 {
        Solution::dfs(grid, x, y - 1)
      } else {
        0
      }
      + Solution::dfs(grid, x, y + 1)
  }

  pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    // let grid = grid as Vec<Vec<usize>>;
    let mut grid = grid.clone();
    let height = grid.len();
    let width = grid[0].len();
    let mut max = 0;

    for i in 0..height {
      for j in 0..width {
        max = cmp::max(max, Solution::dfs(&mut grid, i, j));
      }
    }

    max
  }
}

#[test]
fn case() {
  assert_eq!(
    Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
    true
  );
  assert_eq!(
    Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
    true
  );
  assert_eq!(
    Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
    false
  );
}

fn main() {
  // let sol = Solution;"jjhafiecg"
  // "gj"
  // trace_macros!(true);
  println!(
    "{}",
    Solution::max_area_of_island(vec2d![
      [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
      [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
      [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
    ])
  );
}
