
/*
 * @lc app=leetcode.cn id=34 lang=rust
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */
impl Solution {
  pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
      return vec![-1, -1];
    };
    if nums.len() == 1 {
      if nums[0] == target {
        return vec![0, 0];
      } else {
        return vec![-1, -1];
      }
    }

    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut find_mid = None;

    while low <= high {
      if low == high {
        if target == nums[low] {
          find_mid = Some(low);
        }

        break;
      }
      let mid = (high - low) / 2 + low;
      // println!("{}", mid);
      if nums[mid] == target {
        find_mid = Some(mid);
        break;
      } else if nums[mid] < target {
        low = mid + 1;
      } else {
        high = mid.saturating_sub(1);
      }
    }

    match find_mid {
      Some(mid_idx) => {
        let mut tmp_high = mid_idx;
        let mut tmp_low = mid_idx;
        while tmp_high < nums.len() - 1 && nums[tmp_high + 1] == target {
          tmp_high += 1;
        }

        while tmp_low > 0 && nums[tmp_low - 1] == target {
          tmp_low -= 1;
        }

        vec![tmp_low as i32, tmp_high as i32]
      }
      None => vec![-1, -1],
    }
  }
}

#[test]
fn case() {
  assert_eq!(
    Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
    vec![3, 4]
  );
  assert_eq!(
    Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
    vec![-1, -1]
  );
  assert_eq!(Solution::search_range(vec![1], 0), vec![-1, -1]);
}
