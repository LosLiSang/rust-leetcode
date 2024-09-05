/*
 * @lc app=leetcode.cn id=42 lang=rust

*/
// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut height = height;
        height.push(0);
        height.insert(0, 0); 
        let len = height.len();
        let mut res = 0;
        for i in 1..len{
            let mut left = -1;
            let mut right = -1;
            for j in 0..i{
                left = left.max(height[j]);
            }
            for k in i+1..len{
                right = right.max(height[k]);
            }
            if left.min(right) - height[i] > 0{
                res += left.min(right) - height[i];
            }
        }   
        res
    }
}
// @lc code=end

