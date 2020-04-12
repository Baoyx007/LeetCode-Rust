impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut ret = vec![];

        for target in &words {
            for s in &words {
                if s != target && s.contains(target) {
                    ret.push(target.clone().to_string());
                }
            }
        }

        ret.dedup();
        ret
    }
}
