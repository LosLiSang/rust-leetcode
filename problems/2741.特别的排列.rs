/*
 * @lc app=leetcode.cn id=2741 lang=rust
 *
 * [2741] 特别的排列
 *
 * https://leetcode.cn/problems/special-permutations/description/
 *
 * algorithms
 * Medium (37.90%)
 * Likes:    71
 * Dislikes: 0
 * Total Accepted:    12.5K
 * Total Submissions: 25.2K
 * Testcase Example:  '[2,3,6]'
 *
 * 给你一个下标从 0 开始的整数数组 nums ，它包含 n 个 互不相同 的正整数。如果 nums
 * 的一个排列满足以下条件，我们称它是一个特别的排列：
 * 
 * 
 * 对于 0 <= i < n - 1 的下标 i ，要么 nums[i] % nums[i+1] == 0 ，要么 nums[i+1] % nums[i]
 * == 0 。
 * 
 * 
 * 请你返回特别排列的总数目，由于答案可能很大，请将它对 10^9 + 7 取余 后返回。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：nums = [2,3,6]
 * 输出：2
 * 解释：[3,6,2] 和 [2,6,3] 是 nums 两个特别的排列。
 * 
 * 
 * 示例 2：
 * 
 * 输入：nums = [1,4,3]
 * 输出：2
 * 解释：[3,1,4] 和 [4,1,3] 是 nums 两个特别的排列。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 2 <= nums.length <= 14
 * 1 <= nums[i] <= 10^9
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {

    }
}
// @lc code=end

