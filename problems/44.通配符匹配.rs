/*
 * @lc app=leetcode.cn id=44 lang=rust
 *
 * [44] 通配符匹配
 *
 * https://leetcode.cn/problems/wildcard-matching/description/
 *
 * algorithms
 * Hard (34.03%)
 * Likes:    1147
 * Dislikes: 0
 * Total Accepted:    157.9K
 * Total Submissions: 464.1K
 * Testcase Example:  '"aa"\n"a"'
 *
 * 给你一个输入字符串 (s) 和一个字符模式 (p) ，请你实现一个支持 '?' 和 '*' 匹配规则的通配符匹配：
 * 
 * 
 * '?' 可以匹配任何单个字符。
 * '*' 可以匹配任意字符序列（包括空字符序列）。
 * 
 * 
 * 
 * 
 * 判定匹配成功的充要条件是：字符模式必须能够 完全匹配 输入字符串（而不是部分匹配）。
 * 
 * 
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
 * 示例 2：
 * 
 * 
 * 输入：s = "aa", p = "*"
 * 输出：true
 * 解释：'*' 可以匹配任意字符串。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "cb", p = "?a"
 * 输出：false
 * 解释：'?' 可以匹配 'c', 但第二个 'a' 无法匹配 'b'。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= s.length, p.length <= 2000
 * s 仅由小写英文字母组成
 * p 仅由小写英文字母、'?' 或 '*' 组成
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let slen = s.len();
        let plen = p.len();
        let char_at  = |s: &String, idx|{
            match idx {
                0 => '0',
                idx => s.chars().nth(idx - 1).unwrap()
            }
        };
        let s_at = |idx| char_at(&s, idx);
        let p_at = |idx| char_at(&p, idx);
        let mut dp = vec![vec![false; slen+1]; plen+1];
        dp[0][0] = true;
        for i in 1..plen+1{
            if p_at(i) == '*'{
                dp[i][0] = dp[i-1][0];
            }
        }
        for i in 1..plen+1{
            for j in 1..slen+1{
                if p_at(i) == s_at(j) || p_at(i) == '?'{
                    dp[i][j] = dp[i-1][j-1];                     
                }else if p_at(i) == '*'{ // *
                    dp[i][j] |= dp[i-1][j]; // *匹配0个
                    dp[i][j] |= dp[i-1][j-1]; // *匹配1个
                    dp[i][j] |= dp[i][j-1]; // *匹配多个
                }else{
                    //匹配失败 就是false
                }
            }
        }
        dp[plen][slen]
    }
}
// @lc code=end

