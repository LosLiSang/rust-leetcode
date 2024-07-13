/*
 * @lc app=leetcode.cn id=556 lang=rust
 *
 * [556] 下一个更大元素 III
 *
 * https://leetcode.cn/problems/next-greater-element-iii/description/
 *
 * algorithms
 * Medium (36.85%)
 * Likes:    363
 * Dislikes: 0
 * Total Accepted:    51.1K
 * Total Submissions: 138.7K
 * Testcase Example:  '12'
 *
 * 给你一个正整数 n ，请你找出符合条件的最小整数，其由重新排列 n 中存在的每位数字组成，并且其值大于 n 。如果不存在这样的正整数，则返回 -1 。
 * 
 * 注意 ，返回的整数应当是一个 32 位整数 ，如果存在满足题意的答案，但不是 32 位整数 ，同样返回 -1 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 12
 * 输出：21
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 21
 * 输出：-1
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 
 * 
 */

// @lc code=start
impl Solution {
    fn dfs(depth: usize, cur: i32, vis: &mut Vec<bool>, bit: &Vec<i32>, original: i32) -> i64 {
        if depth == bit.len() {
            return if cur > original { cur as i64 } else { i64::MAX };
        }
        let mut res = i64::MAX;
        for i in 0..vis.len() {
            if !vis[i] {
                let new_cur = cur * 10 + bit[i];
                vis[i] = true;
                res = res.min(Self::dfs(depth + 1, new_cur, vis, bit, original));
                vis[i] = false;
            }
        }
        res
    }

    pub fn next_greater_element(n: i32) -> i32 {
        let mut bit: Vec<i32> = Vec::new();
        let original = n;
        let mut n = n;
        while n != 0 {
            bit.push(n % 10);
            n /= 10;
        }
        bit.reverse(); // 修正位数顺序

        let mut vis: Vec<bool> = vec![false; bit.len()];
        let res = Self::dfs(0, 0, &mut vis, &bit, original);
        if res != i64::MAX {res as i32 } else  {-1}
    }
}

// @lc code=end

