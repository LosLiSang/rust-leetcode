impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let char_list: Vec<char> = s.chars().collect();
        let mut res = "".to_string();
        if char_list.len() <= 1 {return  s}
        let mut flag = true;
        let mut last_two = false;
        let mut i = 0;
        while i < char_list.len() - 1{
            let c1 = char_list[i];
            let c2 = char_list[i+1];
            let d1 = c1.to_digit(10).unwrap();
            let d2 = c2.to_digit(10).unwrap();
            if flag && d1 % 2 == d2 % 2 && d1 > d2 {
                res.push(c2);
                res.push(c1);
                flag = false;
                
                if i == char_list.len() - 2{
                    last_two = true;
                }
                i += 2;
            }else{
                res.push(c1);
                i += 1;
            }
        }
        if last_two {
            return res;
        }
        res.push(*char_list.last().unwrap());
        res
    }
}