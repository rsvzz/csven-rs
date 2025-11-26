pub struct ChangeChar;

impl ChangeChar {
    pub fn new() -> Self {
        ChangeChar
    }

    pub fn get_change_name_for_char(&self, name: String, compare: String, ch: char) -> String {
        let mut verb = String::new();
        let mut status: bool = false;
        for (i, _ch) in name.char_indices() {
            if let Some(compare_ch) = compare.chars().nth(i) {
                if _ch == compare_ch && !status {
                    verb.push(ch);
                } else {
                    verb.push(_ch);
                    status = true;
                }
            } else {
                verb.push(_ch);
                status = true;
            }
        }

        verb
    }
}
