/*
 * @lc app=leetcode.cn id=718 lang=rust
 *
 * [718] 最长重复子数组
 */

// @lc code=start
impl Solution {
    /// dp[i][j]  表示以nums1[i]为末尾 nums2[j]为末尾的最长公共子串
    /// dp[i][j] = if nums1[i] == nums[j] dp[i-1][j-1] + 1
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut res:i32 = 0;
        let len2 = nums2.len();
        let mut dp = vec![0; len2 + 1];
        for (i, num1) in nums1.iter().enumerate(){
            for j in 0..len2{
                let dp_j = len2 - j;
                let num2 = nums2[dp_j - 1];
                if *num1 == num2{
                    dp[dp_j] = dp[dp_j - 1] + 1;
                }else{
                    dp[dp_j] = 0;
                }
                res = res.max(dp[dp_j]);
            }

        }
        res
    }
}

// @lc code=end

