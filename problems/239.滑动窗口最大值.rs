/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 *
 * https://leetcode.cn/problems/sliding-window-maximum/description/
 *
 * algorithms
 * Hard (48.94%)
 * Likes:    2860
 * Dislikes: 0
 * Total Accepted:    672.6K
 * Total Submissions: 1.4M
 * Testcase Example:  '[1,3,-1,-3,5,3,6,7]\n3'
 *
 * 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k
 * 个数字。滑动窗口每次只向右移动一位。
 * 
 * 返回 滑动窗口中的最大值 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
 * 输出：[3,3,5,5,6,7]
 * 解释：
 * 滑动窗口的位置                最大值
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       3
 * ⁠1 [3  -1  -3] 5  3  6  7       3
 * ⁠1  3 [-1  -3  5] 3  6  7       5
 * ⁠1  3  -1 [-3  5  3] 6  7       5
 * ⁠1  3  -1  -3 [5  3  6] 7       6
 * ⁠1  3  -1  -3  5 [3  6  7]      7
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [1], k = 1
 * 输出：[1]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 * 1 <= k <= nums.length
 * 
 * 
 */

// @lc code=start
use std::collections::BTreeMap;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut set = BTreeMap::new();
        let mut res = Vec::new();
        let (mut left , mut right):(usize, usize) = (0, 0);
        let len = nums.len(); 
        let k = k as usize;
        while right < k  {
            *set.entry(nums[right]).or_insert(0) += 1;
            right += 1;
        }
        while right < len {
            if let Some((k, _)) = set.last_key_value(){
                res.push(*k);
            }
            // println!("{:?}", set);
            *set.entry(nums[right]).or_insert(0) += 1;
            if *set.entry(nums[left]).or_default() == 1 {
                set.remove(&nums[left]);
            }else{
                *set.entry(nums[left]).or_default() -= 1;
            }
            right += 1;
            left += 1;
        }
        if let Some((k, _)) = set.last_key_value(){
            res.push(*k);
        }
        res
    }
}
// @lc code=end

