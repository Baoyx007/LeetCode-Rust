impl Solution {
    // const escape =

    pub fn entity_parser(text: String) -> String {
        // let ret = String::new();
        // let mut text_chars = text.chars();

        // let i = 0;
        // while i < text.len() {
        //     let c = text_chars.nth(i).unwrap();

        //     if c != '&' {
        //         continue;
        //     }
        // }
        // ret

        text.replace("&quot;", "\"")
            .replace("&apos;", "'")
            .replace("&amp;", "&")
            .replace("&gt;", ">")
            .replace("&lt;", "<")
            .replace("&frasl;", "/")
    }
}
