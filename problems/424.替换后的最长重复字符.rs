/*
 * @lc app=leetcode.cn id=424 lang=rust
 *
 * [424] 替换后的最长重复字符
 *
 * https://leetcode.cn/problems/longest-repeating-character-replacement/description/
 *
 * algorithms
 * Medium (55.04%)
 * Likes:    878
 * Dislikes: 0
 * Total Accepted:    101.5K
 * Total Submissions: 183.8K
 * Testcase Example:  '"ABAB"\n2'
 *
 * 给你一个字符串 s 和一个整数 k 。你可以选择字符串中的任一字符，并将其更改为任何其他大写英文字符。该操作最多可执行 k 次。
 * 
 * 在执行上述操作后，返回 包含相同字母的最长子字符串的长度。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "ABAB", k = 2
 * 输出：4
 * 解释：用两个'A'替换为两个'B',反之亦然。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "AABABBA", k = 1
 * 输出：4
 * 解释：
 * 将中间的一个'A'替换为'B',字符串变为 "AABBBBA"。
 * 子串 "BBBB" 有最长重复字母, 答案为 4。
 * 可能存在其他的方法来得到同样的结果。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 10^5
 * s 仅由大写英文字母组成
 * 0 <= k <= s.length
 * 
 * 
 */

// @lc code=start
use std::collections::BTreeMap;


impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let k = k as usize;
        let len = s.len();
        let chars: Vec<_> = s.chars().collect();       
        let mut cnt: BTreeMap<char, usize> = BTreeMap::new();
        let (mut left, mut right): (usize, usize) = (0, 0);
        *cnt.entry(chars[right]).or_insert(0) += 1;
        right += 1;
        let mut answer = 1;
        while right < len {
            let mut value = cnt.values().max().unwrap();
            while value + k >= right - left && right < len{
                println!("{:?} {:?} {:?}", cnt, left, right);
                answer = answer.max(right - left);
                *cnt.entry(chars[right]).or_insert(0) += 1;
                right += 1;
                value = cnt.values().max().unwrap();
            }
            if value + k >= right - left {
                answer = answer.max(right - left);
            }
            if *cnt.entry(chars[left]).or_default() == 1 {
                cnt.remove(&chars[left]);
            }else {
                *cnt.entry(chars[left]).or_default() -= 1;
            }
            left += 1;
        } 
        answer as i32
    }
}
// @lc code=end

