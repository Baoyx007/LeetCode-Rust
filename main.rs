use leetcode::vec2d;
struct Solution;

/*
 * @lc app=leetcode.cn id=48 lang=rust
 *
 * [48] 旋转图像
 */
impl Solution {
  fn swap(matrix: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    let tmp = matrix[i][j];
    matrix[i][j] = matrix[j][i];
    matrix[j][i] = tmp;
  }

  pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let len = matrix.len();

    if len <= 1 {
      return;
    }

    for i in 0..len {
      for j in i..len {
        Solution::swap(matrix, i, j);
      }
    }

    for row in matrix.iter_mut() {
      row.reverse();
    }
  }
}

fn main() {
  // println!("{:?}", Solution::generate_parenthesis(3));
  let mut arr = vec2d![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
  Solution::rotate(&mut arr);
  println!("{:?}", arr);
}
