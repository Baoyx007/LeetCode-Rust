/*
 * @lc app=leetcode.cn id=890 lang=rust
 *
 * [890] 查找和替换模式
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();

        for word in words {
            if Solution::match_word(word.as_str(), pattern.as_str()) {
                ret.push(word);
            }
        }

        ret
    }

    fn match_word(word: &str, pattern: &str) -> bool {
        // let mut ret = false;

        let mut forward: HashMap<char, char> = HashMap::new();
        let mut backward: HashMap<char, char> = HashMap::new();

        for (idx, ch) in word.chars().enumerate() {
            let pattern_ch = pattern.chars().nth(idx).unwrap();

            // dbg!(ch, pattern_ch);
            if !forward.contains_key(&ch) {
                forward.insert(ch, pattern_ch);
            }

            if !backward.contains_key(&pattern_ch) {
                backward.insert(pattern_ch, ch);
            }

            // match (forward.get(&ch), backward.get(&pattern_ch)) {
            //     (Some(p1), Some(p2)) => {}
            //     _ => return false,
            // }
            if forward.get(&ch).unwrap() != &pattern_ch || backward.get(&pattern_ch).unwrap() != &ch
            {
                return false;
            }
        }

        true
    }
}

// @lc code=end
