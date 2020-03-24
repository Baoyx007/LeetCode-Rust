use std::cmp;

/// dp[0] = nums[0]
/// dp[1] = max(nums[1], nums[0])
/// dp[n] = max(dp[n-1], dp[n-2] + nums[n]) when n>=2

impl Solution {
    pub fn massage(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return cmp::max(*nums.get(0).unwrap_or(&0), *nums.get(1).unwrap_or(&0));
        }

        let mut dp: Vec<i32> = vec![nums[0], cmp::max(nums[0], nums[1])];
        for (idx, val) in nums.iter().enumerate().skip(2) {
            dbg!(idx, val, &dp);
            dp.push(cmp::max(dp[idx - 1], dp[idx - 2] + val));
        }

        dp[nums.len() - 1]
    }
}
