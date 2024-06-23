
pub struct Solution;

impl Solution {
    /// 假设dp[i] 表示前i个房间能得到的最多的钱
    /// 那么多一个房间, 要么去偷这个房间, 要么不去偷
    
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        match len {
            1 => return nums[0],
            2 => return nums[1].max(nums[0]),
            _ => ()
        }
        let (mut have1, mut have2) = (nums[1], nums[1].max(nums[2])); 
        let (mut none1, mut none2) = (nums[0], nums[0].max(nums[1])); 
        
        for i in 3..len{
            // 如果要抢劫i的房子
            (have1, have2) = (have2, have2.max(have1 + nums[i]));
            // 如果不抢劫i的房子
            (none1, none2) = (none2, none2.max(none1 + nums[i-1]));
        }
        have2.max(none2)
    }
}