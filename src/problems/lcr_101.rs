

pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let sum: i32 = nums.iter().sum();
        if nums.len() == 1 || sum % 2 != 0{
            return false;
        }
        let target: usize = sum as usize / 2;
        let mut dp: Vec<Vec<bool>> = vec![vec![false; 1 + target]; n + 1 as usize];  
        for i in 0..n+1{
            dp[i][0] = true;
        }
        for i in 1..n+1 {
            for j in 1..target+1{
                let r_j = target + 1 - j;
                if dp[i-1][r_j] || nums[i - 1] <= r_j as i32 && dp[i-1][r_j - nums[i - 1] as usize]  {
                    dp[i][r_j] = true;
                }else{
                    dp[i][r_j] = false;
                }
            }
        }
        dp[n][target]
        
    }
}


pub struct Solution1;


// 问题在于不知道从何处开始迭代


impl Solution1 {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut nums_copied = nums.clone();
        nums_copied.sort();
        let (mut l_sum, mut r_sum) = (0, 0);
        let sum: i32 = nums.iter().sum();
        if nums.len() == 1 || sum % 2 != 0{
            return false;
        }
        let mut cur:usize = nums.len() - 1; 
        l_sum = l_sum + nums_copied[cur];
        cur -= 1;
        r_sum = r_sum + nums_copied[cur];
        
        while cur > 0{ // 只要有一个为0 则不再继续循环
            if l_sum < r_sum{
                cur -= 1;
                l_sum = l_sum + nums_copied[cur];
            }else{
                cur -= 1;
                r_sum = r_sum + nums_copied[cur];
            }

        }
        l_sum == r_sum 
    }   
}