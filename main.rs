struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        // let ret = s.clone();

        let mut num = 0;

        for (idx, c) in s.chars().rev().enumerate() {
            dbg!(c.to_digit(2).unwrap());
            num += c.to_digit(2).unwrap() * uszie::pow(2, idx);
        }
        dbg!(num);

        // while ret != "1" {
        //     if ret.chars().last().unwrap() == '1' {

        //     }
        // }

        0
    }
}

fn main() {
    println!("{:?}", Solution::num_steps("1101".to_owned()));
}
