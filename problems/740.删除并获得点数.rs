/*
 * @lc app=leetcode.cn id=740 lang=rust
 *
 * [740] 删除并获得点数
 */

// @lc code=start
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let &max = nums.iter().max().unwrap();
        let mut nums_set = vec![0; max as usize + 1];
        let mut dp = vec![0; max as usize + 1];
        for num in nums{
            nums_set[num as usize] += num;
        }
        dp[1] = nums_set[1];
        for i in 2..max+1{
            let i = i as usize;
            dp[i] = dp[i-1].max(dp[i-2] + nums_set[i]);
        }
        dp[max as usize]
    }
}
// @lc code=end

