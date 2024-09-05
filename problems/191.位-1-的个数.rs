/*
 * @lc app=leetcode.cn id=191 lang=rust
 *
 * [191] 位1的个数
 */

// @lc code=start
impl Solution {
    

    pub fn hamming_weight(n: i32) -> i32 {
        let mut res = 0;
        let mut n = n;
        while n > 0{
            if n % 2 == 1 {res += 1}
            n /= 2;
        }
        res
    }
}
// @lc code=end

