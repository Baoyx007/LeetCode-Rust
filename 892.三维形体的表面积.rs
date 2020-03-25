/*
 * @lc app=leetcode.cn id=892 lang=rust
 *
 * [892] 三维形体的表面积
 */

// @lc code=start
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let dx: Vec<i32> = vec![0, 1, 0, -1];
        let dy: Vec<i32> = vec![-1, 0, 1, 0];

        let mut ret = 0;

        for (idx, row) in grid.iter().enumerate() {
            for (idy, &cell) in row.iter().enumerate() {
                if cell > 0 {
                    ret += 2;

                    for i in 0..4 {
                        let (x, y): (i32, i32) = (idx as i32 + dx[i], idy as i32 + dy[i]);
                        let mut target = 0;
                        if x >= 0 && x < grid.len() as i32 && y >= 0 && y < row.len() as i32 {
                            target = grid[x as usize][y as usize];
                        }
                        ret += std::cmp::max(cell - target, 0);
                    }
                }
            }
        }

        ret
    }
}
// @lc code=end
