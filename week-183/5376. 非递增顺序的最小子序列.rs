impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }

        let sum: i32 = nums.iter().sum();
        let mut sort_nums: Vec<i32> = nums.clone();
        sort_nums.sort();
        sort_nums.reverse();
        // dbg!(&sort_nums);

        let mut ret: Vec<i32> = Vec::new();
        let mut mid_sum = 0;
        // let start = 0usize;
        for n in sort_nums {
            mid_sum += n;
            ret.push(n);
            if mid_sum * 2 > sum {
                break;
            }
        }

        ret
    }
}
