use std::cmp::{max, min};
impl Solution {
    fn inside(x1: i32, y1: i32, x2: i32, y2: i32, xk: i32, yk: i32) -> bool {
        // 若与 x 轴平行，只需要判断 x 的部分
        // 若与 y 轴平行，只需要判断 y 的部分
        // 若为普通线段，则都要判断
        return (x1 == x2 || (min(x1, x2) <= xk && xk <= max(x1, x2)))
            && (y1 == y2 || (min(y1, y2) <= yk && yk <= max(y1, y2)));
    }

    fn update(ans: &mut Vec<f64>, xk: f64, yk: f64) {
        // 将一个交点与当前 ans 中的结果进行比较
        // 若更优则替换
        if ans.len() <= 0 || xk < ans[0] || (xk == ans[0] && yk < ans[1]) {
            ans.splice(..2, vec![xk, yk]);
        }
    }

    pub fn intersection(
        start1: Vec<i32>,
        end1: Vec<i32>,
        start2: Vec<i32>,
        end2: Vec<i32>,
    ) -> Vec<f64> {
        let mut ret: Vec<f64> = vec![];

        let (x1, y1) = (start1[0], end1[0]);
        let (x2, y2) = (start1[1], end1[1]);
        let (x3, y3) = (start2[0], end2[0]);
        let (x4, y4) = (start2[1], end2[1]);

        let k1 = (y2 - y1) * (x4 - x3);
        let k2 = (y4 - y3) * (x2 - x1);

        if k1 == k2 {
            if (y2 - y1) * (x3 - x1) == (y3 - y1) * (x2 - x1) {
                // 判断 (x3, y3) 是否在「线段」(x1, y1)~(x2, y2) 上
                if Self::inside(x1, y1, x2, y2, x3, y3) {
                    Self::update(&mut ret, x3 as f64, y3 as f64);
                }
                // 判断 (x4, y4) 是否在「线段」(x1, y1)~(x2, y2) 上
                if Self::inside(x1, y1, x2, y2, x4, y4) {
                    Self::update(&mut ret, x4 as f64, y4 as f64);
                }
                // 判断 (x1, y1) 是否在「线段」(x3, y3)~(x4, y4) 上
                if Self::inside(x3, y3, x4, y4, x1, y1) {
                    Self::update(&mut ret, x1 as f64, y1 as f64);
                }
                // 判断 (x2, y2) 是否在「线段」(x3, y3)~(x4, y4) 上
                if Self::inside(x3, y3, x4, y4, x2, y2) {
                    Self::update(&mut ret, x2 as f64, y2 as f64);
                }
            }
        } else {
            // 联立方程得到 t1 和 t2 的值
            let t1 = (x3 * (y4 - y3) + y1 * (x4 - x3) - y3 * (x4 - x3) - x1 * (y4 - y3)) as f64
                / ((x2 - x1) * (y4 - y3) - (x4 - x3) * (y2 - y1)) as f64;
            let t2 = (x1 * (y2 - y1) + y3 * (x2 - x1) - y1 * (x2 - x1) - x3 * (y2 - y1)) as f64
                / ((x4 - x3) * (y2 - y1) - (x2 - x1) * (y4 - y3)) as f64;
            // 判断 t1 和 t2 是否均在 [0, 1] 之间
            if t1 >= 0.0 && t1 <= 1.0 && t2 >= 0.0 && t2 <= 1.0 {
                ret = vec![
                    x1 as f64 + t1 * (x2 - x1) as f64,
                    y1 as f64 + t1 * (y2 - y1) as f64,
                ];
            }
        }

        ret
    }
}
