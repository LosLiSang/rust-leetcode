/*
 * @lc app=leetcode.cn id=258 lang=rust
 *
 * [258] 各位相加
 *
 * https://leetcode.cn/problems/add-digits/description/
 *
 * algorithms
 * Easy (70.52%)
 * Likes:    684
 * Dislikes: 0
 * Total Accepted:    203.2K
 * Total Submissions: 288.2K
 * Testcase Example:  '38'
 *
 * 给定一个非负整数 num，反复将各个位上的数字相加，直到结果为一位数。返回这个结果。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: num = 38
 * 输出: 2 
 * 解释: 各位相加的过程为：
 * 38 --> 3 + 8 --> 11
 * 11 --> 1 + 1 --> 2
 * 由于 2 是一位数，所以返回 2。
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入: num = 0
 * 输出: 0
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= num <= 2^31 - 1
 * 
 * 
 * 
 * 
 * 进阶：你可以不使用循环或者递归，在 O(1) 时间复杂度内解决这个问题吗？
 * 
 */

// @lc code=start
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num / 10 == 0 {
            return num;
        }
        let mut num = num;
        let mut res = 0;
        while num != 0{
            res += num % 10;
            num /= 10;
        }
        Self::add_digits(res)
    }
}
// @lc code=end
