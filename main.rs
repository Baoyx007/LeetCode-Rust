
struct Solution;

/*
 * @lc app=leetcode.cn id=695 lang=rust
 *
 * [695] 岛屿的最大面积
 */
impl Solution {

    fn dfs(x:i32,y:i32)->i32{
      
    }

    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        
    }
}




#[test]
fn case() {
  assert_eq!(
    Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
    true
  );
  assert_eq!(
    Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
    true
  );
  assert_eq!(
    Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
    false
  );
}

fn main() {
  // let sol = Solution;"jjhafiecg"
  // "gj"
  println!(
    "{}",
    Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3)
  );
}
