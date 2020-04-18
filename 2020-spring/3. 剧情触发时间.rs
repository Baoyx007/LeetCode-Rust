impl Solution {
    pub fn get_trigger_time(increase: Vec<Vec<i32>>, requirements: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![-1i32; requirements.len()];

        let mut start = vec![0, 0, 0];
        let mut hasReq = 0;

        let mut new_requirements = requirements.clone();
        for (day, inc) in increase.iter().enumerate() {
            // let [c, r, h] = inc.;

            // dbg!(&start);
            let mut left_requirements: Vec<Vec<i32>> = Vec::new();
            for (req_idx, req) in new_requirements.iter().enumerate() {
                // if ret[req_idx] != -1 {
                //     continue;
                // }

                if req[0] <= start[0] && req[1] <= start[1] && req[2] <= start[2] {
                    ret[req_idx] = day as i32 + hasReq;
                    hasReq += 1;
                    continue;
                } else {
                    left_requirements.push(req.clone())
                }
            }
            new_requirements = left_requirements;

            start = vec![start[0] + inc[0], start[1] + inc[1], start[2] + inc[2]];
            dbg!(&new_requirements, &start, day, hasReq);
            if day == increase.len() - 1 {
                for (req_idx, req) in new_requirements.iter().enumerate() {
                    // if ret[req_idx] != -1 {
                    //     continue;
                    // }

                    if req[0] <= start[0] && req[1] <= start[1] && req[2] <= start[2] {
                        ret[req_idx] = day as i32 + hasReq + 1;
                        hasReq += 1;
                        continue;
                    }
                }

                dbg!(&new_requirements, &start, day, hasReq);
            }
        }

        ret
    }
}
