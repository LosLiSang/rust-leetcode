pub struct Solution;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        
        let mut hours_set = vec![0; 24];
        for hour in hours{
            let usize_hour = hour as usize;
            hours_set[usize_hour % 24] += 1;
        }
        let mut res: i64 = 0;
        for i in 1..12{
            res += hours_set[i] as i64 * hours_set[24-i] as i64;
        }
        res += hours_set[0] as i64 * (hours_set[0] as i64 - 1) / 2;
        res += hours_set[12] as i64 * (hours_set[12] as i64 - 1) / 2;
        res       
    }
}