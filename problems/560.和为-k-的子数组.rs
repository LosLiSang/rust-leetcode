/*
 * @lc app=leetcode.cn id=560 lang=rust
 *
 * [560] 和为 K 的子数组
 *
 * https://leetcode.cn/problems/subarray-sum-equals-k/description/
 *
 * algorithms
 * Medium (44.12%)
 * Likes:    2463
 * Dislikes: 0
 * Total Accepted:    529.3K
 * Total Submissions: 1.2M
 * Testcase Example:  '[1,1,1]\n2'
 *
 * 给你一个整数数组 nums 和一个整数 k ，请你统计并返回 该数组中和为 k 的子数组的个数 。
 * 
 * 子数组是数组中元素的连续非空序列。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,1,1], k = 2
 * 输出：2
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [1,2,3], k = 3
 * 输出：2
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 2 * 10^4
 * -1000 <= nums[i] <= 1000
 * -10^7 <= k <= 10^7
 * 
 * 
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        // let mut prefix_sum = vec![0; nums.len() + 1];
        let mut cnt = HashMap::new();
        let mut prefix_sum = 0;
        cnt.insert(0, 1); 
        let mut answer = 0;
        for (i, num) in nums.iter().enumerate(){
            prefix_sum = prefix_sum + num;
            if cnt.contains_key(&(prefix_sum - k)) {
                answer += cnt.get(&(prefix_sum - k)).unwrap();
            }
            *cnt.entry(prefix_sum).or_insert(0) += 1;
        }
        answer
    }
}
// @lc code=end

