impl Solution {
  pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![vec![]];

    for n in nums {
      let new_ret: Vec<Vec<i32>> = ret
        .clone()
        .into_iter()
        .map(|mut r| {
          r.push(n);
          r
        })
        .collect();

      ret.extend(new_ret.iter().cloned());
      // println!("{:?}", ret);
      // [ret,  ]
    }

    ret
  }
}