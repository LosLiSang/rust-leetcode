/*
 * @lc app=leetcode.cn id=395 lang=rust
 *
 * [395] 至少有 K 个重复字符的最长子串
 *
 * https://leetcode.cn/problems/longest-substring-with-at-least-k-repeating-characters/description/
 *
 * algorithms
 * Medium (52.60%)
 * Likes:    916
 * Dislikes: 0
 * Total Accepted:    97.7K
 * Total Submissions: 185.4K
 * Testcase Example:  '"aaabb"\n3'
 *
 * 给你一个字符串 s 和一个整数 k ，请你找出 s 中的最长子串， 要求该子串中的每一字符出现次数都不少于 k 。返回这一子串的长度。
 * 
 * 如果不存在这样的子字符串，则返回 0。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "aaabb", k = 3
 * 输出：3
 * 解释：最长子串为 "aaa" ，其中 'a' 重复了 3 次。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "ababbc", k = 2
 * 输出：5
 * 
 * 解释：最长子串为 "ababb" ，其中 'a' 重复了 2 次， 'b' 重复了 3 次。
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 10^4
 * s 仅由小写英文字母组成
 * 1 <= k <= 10^5
 * 
 * 
 */

// @lc code=start

use std::collections::HashMap;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let k = k as usize;
        let len = s.len();
        let mut cnt = HashMap::new();
        let keys: Vec<_> = s.chars().collect();
        let mut answer = 0;
        for i in 0..len {
            cnt.clear();
            for j in i..len.min(i+k){
                *cnt.entry(keys[j]).or_insert(0) += 1;
            }
            
            for j in i+k..len {
                // println!("{:?}", cnt);
                if cnt.values().all(|val| *val >= k) {
                    answer = answer.max(j - i);
                }
                *cnt.entry(keys[j]).or_insert(0) += 1;
            }
            if cnt.values().all(|val| *val >= k) {
                answer = answer.max(len - i);
            }
        }
        answer as i32
    }
}
// @lc code=end

