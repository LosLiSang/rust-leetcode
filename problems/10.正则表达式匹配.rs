/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 *
 * https://leetcode.cn/problems/regular-expression-matching/description/
 *
 * algorithms
 * Hard (30.70%)
 * Likes:    3913
 * Dislikes: 0
 * Total Accepted:    423.1K
 * Total Submissions: 1.4M
 * Testcase Example:  '"aa"\n"a"'
 *
 * 给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
 * 
 * 
 * '.' 匹配任意单个字符
 * '*' 匹配零个或多个前面的那一个元素
 * 
 * 
 * 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "aa", p = "a"
 * 输出：false
 * 解释："a" 无法匹配 "aa" 整个字符串。
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入：s = "aa", p = "a*"
 * 输出：true
 * 解释：因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "ab", p = ".*"
 * 输出：true
 * 解释：".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 20
 * 1 <= p.length <= 20
 * s 只包含从 a-z 的小写字母。
 * p 只包含从 a-z 的小写字母，以及字符 . 和 *。
 * 保证每次出现字符 * 时，前面都匹配到有效的字符
 * 
 * 
 */

// @lc code=start


impl Solution {
    /// dp[i][j] 表示p[0..i] 是否 能匹配 s[0..j] 
    pub fn is_match(s: String, p: String) -> bool {
        let slen = s.len();
        let plen = p.len();
        // let mut i = 0;
        // let mut j = 0;
        let mut dp = vec![vec![false; slen + 1]; plen + 1];
        dp[0][0] = true;
        let get_char = |str: &String, idx: usize| -> char {
            if idx == 0 {
                return '0';
            }
            str.chars().nth(idx - 1).unwrap()
        };
        let s_char_at = |idx| get_char(&s, idx);
        let p_char_at = |idx| get_char(&p, idx);
        dp[0][0] = true;
        for i in 1..plen+1 {
            if p_char_at(i) == '*' {
                dp[i][0] |= dp[i - 2][0];
            }
        }
        for i in 1..plen + 1 {
            for j in 1..slen + 1 {
                if p_char_at(i) == s_char_at(j) || p_char_at(i) == '.' {
                    dp[i][j] |= dp[i - 1][j - 1];
                } else if p_char_at(i) == '*' {
                    dp[i][j] |= p_char_at(i - 1) == s_char_at(j) && dp[i - 1][j - 1];
                    dp[i][j] |= p_char_at(i - 1) == s_char_at(j) && dp[i][j - 1];
                    dp[i][j] |= p_char_at(i - 1) == '.' && dp[i - 1][j - 1]; // 匹配多次
                    dp[i][j] |= p_char_at(i - 1) == '.' && dp[i][j - 1]; // 匹配1次
                    dp[i][j] |= dp[i - 2][j]; // 匹配0次
                }
            }
        }
        dp[plen][slen]
    }
}
// @lc code=end

