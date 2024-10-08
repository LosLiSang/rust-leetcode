/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 *
 * https://leetcode.cn/problems/maximum-subarray/description/
 *
 * algorithms
 * Medium (55.40%)
 * Likes:    6692
 * Dislikes: 0
 * Total Accepted:    1.8M
 * Total Submissions: 3.2M
 * Testcase Example:  '[-2,1,-3,4,-1,2,1,-5,4]'
 *
 * 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
 * 
 * 子数组 是数组中的一个连续部分。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
 * 输出：6
 * 解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [1]
 * 输出：1
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：nums = [5,4,-1,7,8]
 * 输出：23
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 * 
 * 
 * 
 * 
 * 进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
 * 
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let nums_at = |idx| {
            match idx {
                0 => 0,
                idx => nums[idx-1]
            }
        };
        let mut max = i32::MIN;
        // let mut dp = vec![0; len+1];
        let mut dp = 0;
        for i in 1..len+1{
            dp = nums_at(i).max(dp + nums_at(i));
            max = max.max(dp);
        }
        max
    }
}
// @lc code=end

