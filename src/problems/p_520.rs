pub struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut first_cap = true;
        let mut second_cap = true;
        for (i, c) in word.chars().enumerate() {
            if i == 0 {
                first_cap = c.is_ascii_uppercase();
            } else {
                if first_cap  {
                    if i == 1{
                        second_cap = c.is_ascii_uppercase();
                    }else {
                        if second_cap && c.is_ascii_lowercase(){
                            return false;
                        }
                        if !second_cap && c.is_uppercase(){
                            return false;
                        }
                    }
                }
                if !first_cap && c.is_ascii_uppercase() {
                    return false;
                }
            }
        }
        true
    }
}
