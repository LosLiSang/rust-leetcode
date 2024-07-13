/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 * [72] 编辑距离
 *
 * https://leetcode.cn/problems/edit-distance/description/
 *
 * algorithms
 * Medium (62.89%)
 * Likes:    3407
 * Dislikes: 0
 * Total Accepted:    498.2K
 * Total Submissions: 792.2K
 * Testcase Example:  '"horse"\n"ros"'
 *
 * 给你两个单词 word1 和 word2， 请返回将 word1 转换成 word2 所使用的最少操作数  。
 * 
 * 你可以对一个单词进行如下三种操作：
 * 
 * 
 * 插入一个字符
 * 删除一个字符
 * 替换一个字符
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：word1 = "horse", word2 = "ros"
 * 输出：3
 * 解释：
 * horse -> rorse (将 'h' 替换为 'r')
 * rorse -> rose (删除 'r')
 * rose -> ros (删除 'e')
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：word1 = "intention", word2 = "execution"
 * 输出：5
 * 解释：
 * intention -> inention (删除 't')
 * inention -> enention (将 'i' 替换为 'e')
 * enention -> exention (将 'n' 替换为 'x')
 * exention -> exection (将 'n' 替换为 'c')
 * exection -> execution (插入 'u')
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= word1.length, word2.length <= 500
 * word1 和 word2 由小写英文字母组成
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1:Vec<char> = word1.chars().collect();
        let word2:Vec<char> = word2.chars().collect();
        let len1 = word1.len();
        let len2 = word2.len();
        let char_at = |list: &Vec<char>, idx| {
            match idx {
                0 => '.',
                idx => list[idx-1]
            }
        };
        let word1_at = |idx| char_at(&word1, idx);
        let word2_at = |idx| char_at(&word2, idx);
        let mut dp = vec![0; len2+1];
        for i in 1..len2+1{
            dp[i] = i;
        }
        for i in 1..len1+1{
            let mut pre = i;
            dp[0] = i-1;
            for j in 0..len2+1{
                let temp = dp[j];
                if word1_at(i) == word2_at(j){
                    dp[j] = pre;
                }else if word2_at(j) != '.'{
                    dp[j] = dp[j-1].min(dp[j]).min(pre) + 1;
                }else{
                    dp[j] += 1;
                }
                pre = temp;
            }
        }
        dp[len2] as i32
    }
}
// @lc code=end

