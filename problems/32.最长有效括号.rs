/*
 * @lc app=leetcode.cn id=32 lang=rust
 *
 * [32] 最长有效括号
 *
 * https://leetcode.cn/problems/longest-valid-parentheses/description/
 *
 * algorithms
 * Hard (38.08%)
 * Likes:    2517
 * Dislikes: 0
 * Total Accepted:    457.7K
 * Total Submissions: 1.2M
 * Testcase Example:  '"(()"'
 *
 * 给你一个只包含 '(' 和 ')' 的字符串，找出最长有效（格式正确且连续）括号子串的长度。
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "(()"
 * 输出：2
 * 解释：最长有效括号子串是 "()"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = ")()())"
 * 输出：4
 * 解释：最长有效括号子串是 "()()"
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = ""
 * 输出：0
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= s.length <= 3 * 10^4
 * s[i] 为 '(' 或 ')'
 * 
 * 
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let len = s.len();
        let mut max = 0;
        let mut dp = vec![0; len+1];
        let get_char = |idx: usize| -> char{
            match idx {
                0 => '0',
                idx => s.chars().nth(idx-1).unwrap(),  
            }
        };
        for i in 1..len+1{
            if get_char(i) == '('{
                if get_char(i - 1) == ')'{
                    dp[i] = dp[i - 1];
                }else{
                    dp[i] = 0;
                }
            }else if get_char(i) == ')'{
                if get_char(i - 1) == ')' {
                    if get_char(i - 1 - dp[i - 1]) == '('{
                        dp[i] = dp[i-1] + dp[i - 1 - dp[i - 1]];
                        dp[i] += 2;
                    }
                }else if get_char(i - 1) == '('{
                    dp[i] = dp[i-1] + 2;
                }else{
                    dp[i] = 0;
                }
            }
            max = max.max(dp[i]);
        }
        max as i32
    }
}

// @lc code=end

