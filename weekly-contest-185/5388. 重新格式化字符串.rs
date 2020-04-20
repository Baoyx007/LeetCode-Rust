impl Solution {
    pub fn reformat(s: String) -> String {
        let mut ret = String::new();

        let chars: Vec<char> = s.chars().collect();
        let mut p_char = 0;
        let mut p_number: i32 = s.len() as i32 - 1;

        loop {
            // dbg!(&ret);
            while p_char < s.len() && !chars[p_char].is_ascii_alphabetic() {
                p_char += 1;
            }
            if p_char < s.len() {
                ret.push(chars[p_char]);
                p_char += 1;
            } else {
                break;
            }

            while p_number >= 0 && !chars[p_number as usize].is_digit(10) {
                p_number -= 1;
            }
            if p_number >= 0 {
                ret.push(chars[p_number as usize]);
                p_number -= 1;
            } else {
                break;
            }
        }
        let ret_chars: Vec<char> = ret.chars().collect();
        // dbg!(&ret);
        // dbg!(p_char, p_number);
        if p_char >= s.len()
            && p_number >= 0
            && chars[p_number as usize].is_digit(10)
            && ((ret_chars.len() > 1 && ret_chars[0].is_ascii_alphabetic()) || ret_chars.len() == 0)
        {
            ret.insert(0, chars[p_number as usize]);
        } else if p_char < s.len()
            && p_number < 0
            && chars[p_char].is_ascii_alphabetic()
            && ((ret_chars.len() > 1 && ret_chars[ret_chars.len() - 1].is_digit(10))
                || ret_chars.len() == 0)
        {
            ret.push(chars[p_char]);
        }

        if ret.len() == s.len() {
            ret
        } else {
            "".to_owned()
        }
    }
}
