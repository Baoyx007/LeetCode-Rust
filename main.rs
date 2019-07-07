struct Solution;

impl Solution {
  pub fn max_depth_after_split(seq: String) -> Vec<i32> {
    
  }

  pub fn backtrace(seq:String, max:i32,ret:&mut Vec<i32>){
    // validate ()

    ret.push(1)


    ret.push(0)

    Self::backtrace(seq: String, max: i32, ret:  Vec<i32>)
  }
}

fn main() {
  // println!("{:?}", Solution::defang_i_paddr("255.100.50.0".to_owned()));
}
