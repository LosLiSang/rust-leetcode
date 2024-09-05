pub struct Solution;
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let keys:Vec<char> = answer_key.chars().collect();
        fn get(keys: &Vec<char>, k: i32, c: char) -> i32{
            let (mut low, mut high) = (0, 0); // 左闭右开
            let mut answer = 0;
            let mut cnt_t = 0;
            while high < keys.len(){
                if keys[high] == c {cnt_t += 1};
                while cnt_t > k {
                    if keys[low] == c {
                        low += 1
                    }
                }
                answer = answer.max(high - low);
                high += 1;
            }
            answer as i32
        }
        get(&keys, k, 'T').max(get(&keys, k, 'F'))
    }
}
