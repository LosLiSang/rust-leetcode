/*
 * @lc app=leetcode.cn id=503 lang=rust
 *
 * [503] 下一个更大元素 II
 *
 * https://leetcode.cn/problems/next-greater-element-ii/description/
 *
 * algorithms
 * Medium (67.54%)
 * Likes:    975
 * Dislikes: 0
 * Total Accepted:    264K
 * Total Submissions: 389K
 * Testcase Example:  '[1,2,1]'
 *
 * 给定一个循环数组 nums （ nums[nums.length - 1] 的下一个元素是 nums[0] ），返回 nums 中每个元素的
 * 下一个更大元素 。
 * 
 * 数字 x 的 下一个更大的元素 是按数组遍历顺序，这个数字之后的第一个比它更大的数，这意味着你应该循环地搜索它的下一个更大的数。如果不存在，则输出 -1
 * 。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: nums = [1,2,1]
 * 输出: [2,-1,2]
 * 解释: 第一个 1 的下一个更大的数是 2；
 * 数字 2 找不到下一个更大的数； 
 * 第二个 1 的下一个最大的数需要循环搜索，结果也是 2。
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入: nums = [1,2,3,4,3]
 * 输出: [2,3,4,-1,4]
 * 
 * 
 * 
 * 
 * 提示:
 * 
 * 
 * 1 <= nums.length <= 10^4
 * -10^9 <= nums[i] <= 10^9
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let len = nums.len();
        let mut right:Vec<i32> = (0..len as i32).collect();
        let (mut max, mut max_idx) = (i32::MIN, 0);
        for i in 0..len{
            if max < nums[i]{
                max = nums[i];
                max_idx = i;
            }
        }
        let mut nums = nums;
        nums.rotate_right(len - 1 - max_idx);
        for i in (0..len).rev(){
            while !stack.is_empty() && nums[*stack.last().unwrap() as usize] <= nums[i]{
                stack.pop();
            }
            if stack.is_empty(){
                right[i] = -1;
            }else{
                right[i] = nums[*stack.last().unwrap() as usize];
            }
            stack.push(i as i32);
        }
        right.rotate_left(len - 1 - max_idx);
        right
    }
}
// @lc code=end

