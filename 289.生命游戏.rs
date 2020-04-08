use leetcode::vec2d;

struct Solution;

/*
* @lc app=leetcode.cn id=289 lang=rust
*
 0->1 & 2
 1->0 & -1
* [289] 生命游戏
*/

// @lc code=start
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let neighbors: [i32; 3] = [0, 1, -1];

        let row_len = board.len();
        let col_len = board[0].len();

        for row in 0..row_len {
            for col in 0..col_len {
                let mut live_count = 0;

                for i in 0..3 {
                    for j in 0..3 {
                        if i == 0 && j == 0 {
                            continue;
                        }

                        let (target_row, target_col) =
                            (row as i32 + neighbors[i], col as i32 + neighbors[j]);

                        if target_row >= 0
                            && target_row < row_len as i32
                            && target_col >= 0
                            && target_col < col_len as i32
                            && board[target_row as usize][target_col as usize].abs() == 1
                        {
                            live_count += 1;
                        }
                    }
                }
                // dbg!(live_count);
                // 规则 1 或规则 3
                if (board[row][col] == 1) && (live_count < 2 || live_count > 3) {
                    // -1 代表这个细胞过去是活的现在死了
                    board[row][col] = -1;
                }
                // 规则 4
                if board[row][col] == 0 && live_count == 3 {
                    // 2 代表这个细胞过去是死的现在活了
                    board[row][col] = 2;
                }
            }
        }
        // dbg!(&board);

        for row in 0..row_len {
            for col in 0..col_len {
                if board[row][col] > 0 {
                    board[row][col] = 1;
                } else {
                    board[row][col] = 0;
                }
            }
        }
    }
}
