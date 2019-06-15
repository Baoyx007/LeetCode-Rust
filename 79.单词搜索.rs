struct Solution;
use leetcode::{u_add_i, vec2d};

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
  pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    for (i, x) in board.iter().enumerate() {
      for (j, &y) in x.iter().enumerate() {
        // println!("{},{} {} {}", i, j, y, word.chars().nth(0).unwrap());
        if y == word.chars().nth(0).unwrap()
          && Solution::dfs(
            i,
            j,
            &word[1..],
            &board,
            &mut HashSet::from_iter(vec![(i, j)]),
          )
        {
          return true;
        }
      }
    }

    false
    // ret
  }

  /// * `x`  start
  /// * `y` end
  fn dfs(
    x: usize,
    y: usize,
    word: &str,
    board: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
  ) -> bool {
    if word.is_empty() {
      //dfs 结束
      return true;
    }

    let direction: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    for &(i, j) in &direction {
      if let (Some(tmp_x), Some(tmp_y)) =
        (u_add_i(x, i, board.len()), u_add_i(y, j, board[0].len()))
      // 合法 index 递归
      {
        // println!("{},{}", tmp_x, tmp_y);
        // println!("{},{}", tmp_x, tmp_y)
        if !visited.contains(&(tmp_x, tmp_y)) && board[tmp_x][tmp_y] == word.chars().nth(0).unwrap()
        {
          visited.insert((tmp_x, tmp_y));
          if Solution::dfs(tmp_x, tmp_y, &word[1..], board, visited) {
            return true;
          } else {
            visited.remove(&(tmp_x, tmp_y));
          }
        }
      } else {
        // println!(
        //   "{},{},{} {:?} {:?}",
        //   x,
        //   i,
        //   board.len(),
        //   u_add_i(x, i, board.len()),
        //   u_add_i(y, j, board[0].len())
        // );
      }
    }

    false
  }
}

fn main() {
  let board = vec2d![
    ['A', 'B', 'C', 'E'],
    ['S', 'F', 'C', 'S'],
    ['A', 'D', 'E', 'E']
  ];

  println!("{}", Solution::exist(board, "ABCCED".to_string()));
}
