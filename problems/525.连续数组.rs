/*
 * @lc app=leetcode.cn id=525 lang=rust
 *
 * [525] 连续数组
 *
 * https://leetcode.cn/problems/contiguous-array/description/
 *
 * algorithms
 * Medium (54.61%)
 * Likes:    746
 * Dislikes: 0
 * Total Accepted:    81.6K
 * Total Submissions: 149K
 * Testcase Example:  '[0,1]'
 *
 * 给定一个二进制数组 nums , 找到含有相同数量的 0 和 1 的最长连续子数组，并返回该子数组的长度。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: nums = [0,1]
 * 输出: 2
 * 说明: [0, 1] 是具有相同数量 0 和 1 的最长连续子数组。
 * 
 * 示例 2:
 * 
 * 
 * 输入: nums = [0,1,0]
 * 输出: 2
 * 说明: [0, 1] (或 [1, 0]) 是具有相同数量0和1的最长连续子数组。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * nums[i] 不是 0 就是 1
 * 
 * 
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut position = HashMap::new();
        position.insert(0, 0);
        let mut answer: usize = 0;
        let mut cur = 0;
        for (i, num) in nums.iter().enumerate(){
            if *num == 0 {
                cur -= 1;
            }else {
                cur += 1;
            }
            if position.contains_key(&cur) {
                answer = answer.max(i - *position.get(&cur).unwrap() + 1);                
            }else {
                position.insert(cur, i + 1);
            }
        }
        answer as i32
    }
}  
// @lc code=end

