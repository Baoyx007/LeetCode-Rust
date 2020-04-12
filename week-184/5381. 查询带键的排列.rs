impl Solution {
    fn put_head(arr: &mut Vec<i32>, pos: usize) {
        let first = arr[pos];
        for i in (0..pos).rev() {
            arr[i + 1] = arr[i];
        }

        arr[0] = first;
    }

    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut mp: Vec<i32> = (1..=m).collect();
        let mut ret = Vec::new();

        for q in queries {
            let pos = mp.iter().position(|&i| i == q).unwrap();
            ret.push(pos as i32);
            Self::put_head(&mut mp, pos);
        }

        ret
    }
}
