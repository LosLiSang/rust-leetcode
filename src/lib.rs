use std::collections::HashMap;




pub struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = HashMap::new();
        let mut cur = 0;
        cnt.insert(0, 1);
        
        for (i, num) in nums.iter().enumerate() {
            if num % 2 == 1 {
                cur += 1;
            } 
            *cnt.entry(cur).or_insert(0) += 1;
        }
        todo!()
    }
}