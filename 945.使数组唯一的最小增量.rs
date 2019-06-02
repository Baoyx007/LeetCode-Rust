/*
 * @lc app=leetcode.cn id=945 lang=rust
 *
 * [945] 使数组唯一的最小增量
 */
impl Solution {
    pub fn min_increment_for_unique(a: Vec<i32>) -> i32 {
        if a.len() <= 1 {
            return 0;
        }
        let mut a = a.clone();
        a.sort();

        let mut ret = 0;
        let mut pre = a[0];
        for item in a.iter().skip(1) {
            let cur = *item;
            // println!("{}", cur);

            if cur <= pre {
                // println!("cur {}, pre {}", cur, pre);
                pre += 1;
                ret += pre - cur;
                // cur = pre +1;
            } else {
                pre = cur;
            }

        }

        ret
    }

    #[test]
    fn case1() {
        assert_eq!(min_increment_for_unique(vec![3,2,1,2,1,7]), 6);
    }
}


/*

note : 排序下, 计算空的值和重复值的差值
[3,2,1,2,1,7] -> [1 1 2 2 3 7]  [4 5 6]  [8 ~...] 

-> 1=>4 2 =>5 
-> 6
*/

