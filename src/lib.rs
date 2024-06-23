

pub struct Solution;

impl Solution {
    // 引入一个新的股票, 要么买 要么卖 要么啥都不做
    // dp[i]在这一天未持有股票的最高收益
    // dp[i] = (j=[1..i-1])max{dp[j] - nums[j] + nums[j] - fee} 
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32{
        let len = prices.len();
        let mut dp = vec![0; len + 1];
        // dp[2] = 
        for i in 1..len{
            let dp_i = i + 1;
            dp[dp_i] = dp[dp_i].max(dp[dp_i-1]); 
            for j in 0..i{ // 在j天买 在i天卖 
                let dp_j = j + 1;
                dp[dp_i] = dp[dp_i].max(dp[dp_j] - prices[j] + prices[i] - fee);
            } 
            
        }
        dp[len]
    }
}
