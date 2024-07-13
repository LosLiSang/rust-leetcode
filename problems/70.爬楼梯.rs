/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 *
 * https://leetcode.cn/problems/climbing-stairs/description/
 *
 * algorithms
 * Easy (54.63%)
 * Likes:    3541
 * Dislikes: 0
 * Total Accepted:    1.5M
 * Total Submissions: 2.8M
 * Testcase Example:  '2'
 *
 * 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
 * 
 * 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 2
 * 输出：2
 * 解释：有两种方法可以爬到楼顶。
 * 1. 1 阶 + 1 阶
 * 2. 2 阶
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 3
 * 输出：3
 * 解释：有三种方法可以爬到楼顶。
 * 1. 1 阶 + 1 阶 + 1 阶
 * 2. 1 阶 + 2 阶
 * 3. 2 阶 + 1 阶
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= n <= 45
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; n as usize +1];
        dp[0] = 1;
        dp[1] = 1;
        let usize_n = n as usize;
        for i in 2..usize_n+1{
            dp[i] = dp[i-1] + dp[i-2];
        }
        
        dp[usize_n]
    }
}
// @lc code=end

