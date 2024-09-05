/*
 * @lc app=leetcode.cn id=67 lang=rust
 *
 * [67] 二进制求和
 *
 * https://leetcode.cn/problems/add-binary/description/
 *
 * algorithms
 * Easy (53.16%)
 * Likes:    1214
 * Dislikes: 0
 * Total Accepted:    407.4K
 * Total Submissions: 765.8K
 * Testcase Example:  '"11"\n"1"'
 *
 * 给你两个二进制字符串 a 和 b ，以二进制字符串的形式返回它们的和。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入:a = "11", b = "1"
 * 输出："100"
 * 
 * 示例 2：
 * 
 * 
 * 输入：a = "1010", b = "1011"
 * 输出："10101"
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= a.length, b.length <= 10^4
 * a 和 b 仅由字符 '0' 或 '1' 组成
 * 字符串如果不是 "0" ，就不含前导零
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut res = String::from("");
        let long: Vec<char> = if a.len() > b.len() {
            a.clone().chars().collect()
        } else {
            b.clone().chars().collect()
        };
        let mut short: Vec<char> = if a.len() > b.len() {
            b.clone().chars().collect()
        } else {
            a.clone().chars().collect()
        };

        let mut c = '0';
        for _ in 0..long.len() - short.len() {
            short.insert(0, '0');
        }

        for i in 1..long.len() + 1 {
            if long[long.len() - i] == short[short.len() - i] {
                res.insert(0, c);
                c = long[long.len() - i];
            } else {
                if c == '0' {
                    res.insert(0, '1')
                } else {
                    res.insert(0, '0')
                }
            }
        }
        if c == '1' {
            res.insert(0, '1')
        }
        res
    }
}
// @lc code=end
