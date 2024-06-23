use std::cmp::min;


pub struct Solution;

impl Solution {
    /// coins:  输入的面额值, 均不相等
    /// amount: 目标金额
    /// return_value:   组成总金额所需的最少的硬币的个数, 不能则返回-1
    /// dp:     dp[i][j]前i个数额的硬币能表示数值为j的钱的最少硬币数
    /// Transition function: dp[i][j] = min(dp[i-1][j], dp[i-1][j-coin[i]] + 1)
    /// 完全背包问题
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let coins_len = coins.len();
        let mut  dp = vec![vec![i32::MAX; amount as usize + 1]; coins_len + 1] ;
        for i in 0..coins_len+1{
            dp[i][0] = 0;
        }

        for i in 1..coins_len+1{
            for j  in 1..amount+1{
                let usize_j = j as usize;
                let usize_coin = coins[i - 1] as usize;
                if usize_j >= usize_coin && dp[i][usize_j-usize_coin] != i32::MAX{ // 如果可以用这个硬币
                    dp[i][usize_j] = min(dp[i][usize_j], dp[i][usize_j-usize_coin] + 1);
                } 
                if dp[i-1][usize_j] != i32::MAX { //  如果可以不使用这个硬币
                    dp[i][usize_j] = min(dp[i][usize_j], dp[i-1][usize_j]);
                }
            }
        }
        
        match dp[coins_len][amount as usize] {
            i32::MAX => -1,
            res => res
        } 
        
    }
}