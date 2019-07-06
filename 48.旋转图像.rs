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