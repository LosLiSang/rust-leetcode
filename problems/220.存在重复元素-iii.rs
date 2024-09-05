/*
 * @lc app=leetcode.cn id=220 lang=rust
 *
 * [220] 存在重复元素 III
 *
 * https://leetcode.cn/problems/contains-duplicate-iii/description/
 *
 * algorithms
 * Hard (30.61%)
 * Likes:    737
 * Dislikes: 0
 * Total Accepted:    103.2K
 * Total Submissions: 336.3K
 * Testcase Example:  '[1,2,3,1]\n3\n0'
 *
 * 给你一个整数数组 nums 和两个整数 indexDiff 和 valueDiff 。
 * 
 * 找出满足下述条件的下标对 (i, j)：
 * 
 * 
 * i != j,
 * abs(i - j) <= indexDiff
 * abs(nums[i] - nums[j]) <= valueDiff
 * 
 * 
 * 如果存在，返回 true ；否则，返回 false 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,2,3,1], indexDiff = 3, valueDiff = 0
 * 输出：true
 * 解释：可以找出 (i, j) = (0, 3) 。
 * 满足下述 3 个条件：
 * i != j --> 0 != 3
 * abs(i - j) <= indexDiff --> abs(0 - 3) <= 3
 * abs(nums[i] - nums[j]) <= valueDiff --> abs(1 - 1) <= 0
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [1,5,9,1,5,9], indexDiff = 2, valueDiff = 3
 * 输出：false
 * 解释：尝试所有可能的下标对 (i, j) ，均无法满足这 3 个条件，因此返回 false 。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 2 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 * 1 <= indexDiff <= nums.length
 * 0 <= valueDiff <= 10^9
 * 
 * 
 */

// @lc code=start

use std::collections::BTreeSet;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let len = nums.len() as i32; 
        let (mut left, mut right): (usize, usize) = (0, 0); // 左闭右闭
        let mut set =  BTreeSet::new();
        while right < nums.len()  {
            let (low, high) = (nums[right] - value_diff, nums[right] + value_diff);
            // println!("{:?} {:?} {:?}", left, right, set);
            if let Some(temp) = set.range(low..).next() {
                if *temp <= high{
                    return true;
                }
            }
            if right - left >= index_diff as usize {
                set.remove(&nums[left as usize]);
                left += 1;
            }
            set.insert(nums[right]);
            right += 1;  
        }
        false
    }
}
// @lc code=end
