/*
 * @lc app=leetcode.cn id=714 lang=rust
 *
 * [714] 买卖股票的最佳时机含手续费
 */

// @lc code=start
impl Solution {
    // 引入一个新的股票, 要么买 要么卖 要么啥都不做
    // dp[i]在这一天未持有股票的最高收益
    // dp[i] = (j=[1..i-1])max{dp[j] - nums[j] + nums[j] - fee} 
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32{
        let len = prices.len();
        let mut dp = vec![vec![0, 0]; len + 1];
        // dp[2] = 
        dp[1][1] = -prices[0] - fee; 
        for i in 1..len{
            let dp_i = i + 1;
            dp[dp_i][0] = dp[dp_i-1][0].max(dp[dp_i-1][1] + prices[i]); 
            dp[dp_i][1] = dp[dp_i-1][1].max(dp[dp_i-1][0] - prices[i] - fee); 
            
        }
        dp[len][0]
    }
}
// @lc code=end

