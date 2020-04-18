impl Solution {
    fn bt(path: &mut Vec<i32>, relation: &Vec<Vec<i32>>, k: i32, n: i32, ret: &mut Box<i32>) {
        let last_val = path.last().unwrap().clone();
        let target_idx = path.len();
        if target_idx - 1 == k as usize {
            //end
            // dbg!(&path);
            if last_val == n - 1 {
                **ret += 1;
            }
            return;
        }

        // for &option in slice.iter().filter(|&r| r[0] == target_idx as i32) {}
        for opt in relation {
            // dbg!(&opt);
            if opt[0] == last_val {
                //choose
                path.push(opt[1]);
                Solution::bt(path, &relation, k, n, ret);
                path.pop();
            }
        }
    }

    pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ret = Box::new(0);
        let mut path = vec![0];

        Solution::bt(&mut path, &relation, k, n, &mut ret);
        *ret
    }
}
