
pub struct Solution;

impl Solution {

    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max = 0;
        for i in 0..nums.len(){
            if i > max { // 走不到这一步, 自然后面都无法到达了
                return false;
            }

            max = max.max(i + nums[i] as usize); // 表示在第i步能走的最远的距离
        }
        true
    }

}




pub struct Solution1;

impl Solution1 {
    /// input
    /// - nums: 一组非负整数, 数组的每个元素代表在该位置可以跳跃的最大高度
    /// 
    /// output
    /// - 能否到达最后一个下标
    /// 
    /// dp[i] 表示 能否到达i这个位置
    /// dp[i] = [num for num in nums[0..i]]
    
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let nums_len = nums.len();
        let mut dp = vec![false; nums_len];
        dp[0] = true;
        for (i, num) in nums.iter().enumerate(){
            if dp[i] == true{ // 这一点可以走
                let usize_num = *num as usize;
                for j in i..i+usize_num{
                    if j < nums_len - 1{
                        dp[j+1] = true;
                    }
                }                            
            }
        }
        dp[nums_len]
    }
}