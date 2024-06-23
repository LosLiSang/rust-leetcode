
pub struct Solution;

impl Solution {
    /// hours = [72,48,24,3]
    /// (0, 1) (0, 2) (1, 2)
    /// 
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {

        let len = hours.len();
        let mut res = 0;
        for i in 0..len{
            for j in i+1..len{
                if (hours[i] % 24 + hours[j] % 24) % 24 == 0 {
                    res += 1;
                }
            }
        }   
        res
    }
}