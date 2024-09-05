/*
 * @lc app=leetcode.cn id=3144 lang=rust
 *
 * [3144] 分割字符频率相等的最少子字符串
 *
 * https://leetcode.cn/problems/minimum-substring-partition-of-equal-character-frequency/description/
 *
 * algorithms
 * Medium (50.82%)
 * Likes:    30
 * Dislikes: 0
 * Total Accepted:    9.1K
 * Total Submissions: 14.3K
 * Testcase Example:  '"fabccddg"'
 *
 * 给你一个字符串 s ，你需要将它分割成一个或者更多的 平衡 子字符串。比方说，s == "ababcc" 那么 ("abab", "c", "c")
 * ，("ab", "abc", "c") 和 ("ababcc") 都是合法分割，但是 ("a", "bab", "cc") ，("aba", "bc",
 * "c") 和 ("ab", "abcc") 不是，不平衡的子字符串用粗体表示。
 *
 * 请你返回 s 最少 能分割成多少个平衡子字符串。
 *
 * 注意：一个 平衡 字符串指的是字符串中所有字符出现的次数都相同。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "fabccddg"
 *
 * 输出：3
 *
 * 解释：
 *
 * 我们可以将 s 分割成 3 个子字符串：("fab, "ccdd", "g") 或者 ("fabc", "cd", "dg") 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "abababaccddb"
 *
 * 输出：2
 *
 * 解释：
 *
 * 我们可以将 s 分割成 2 个子字符串：("abab", "abaccddb") 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 1000
 * s 只包含小写英文字母。
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {


        let len = s.len();
        let mut dp: Vec<i32> = (0..len as i32 + 1).collect();
        
        let chars: Vec<char> = s.chars().collect();
        for i in 1..len {
            let mut cnt = HashMap::new();
            for j in (0..i+1).rev() {
                *cnt.entry(chars[j]).or_insert(0) += 1;
                if cnt.values().max() == cnt.values().min() {
                    dp[i+1] = dp[i+1].min(dp[j] + 1);
                }
            }
        }
        dp[len]
    }
}
// @lc code=end
