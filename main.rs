struct Solution;

impl Solution {
    // rgb: 1,2,3
    // fn bt(row: i32, col: i32, grid: Vec<Vec<i32>>, count: i32) {}

    // pub fn num_of_ways(n: i32) -> i32 {
    //     let ret = 0;

    //     let mut grid: Vec<Vec<i32>> = vec![];
    //     for row in 0..(n as usize) {
    //         grid.push(vec![0, 0, 0]);
    //         for col in 0..3 {
    //             // choose red
    //             grid[row][col] = 1;
    //             Self::bt();
    //             grid[row][col] = 2;
    //             grid[row][col] = 3;
    //         }
    //     }

    //     ret
    // }

    pub fn num_of_ways(n: i32) -> i32 {
        let mut row = (6u64, 6u64); //abc , aba
        let d_mod = 10u64.pow(9) + 7;

        for _ in 1..n {
            row = (
                (row.0 * 2 + row.1 * 2) % d_mod,
                (row.0 * 2 + row.1 * 3) % d_mod,
            );
        }

        ((row.0 + row.1) % d_mod) as i32
    }
}
fn main() {
    // println!("{}", String::new());
    println!("{:?}", Solution::num_of_ways(5000));
}
