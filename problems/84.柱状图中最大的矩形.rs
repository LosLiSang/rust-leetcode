/*
 * @lc app=leetcode.cn id=84 lang=rust
 *
 * [84] 柱状图中最大的矩形
 *
 * https://leetcode.cn/problems/largest-rectangle-in-histogram/description/
 *
 * algorithms
 * Hard (45.87%)
 * Likes:    2740
 * Dislikes: 0
 * Total Accepted:    425.5K
 * Total Submissions: 927.5K
 * Testcase Example:  '[2,1,5,6,2,3]'
 *
 * 给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。
 * 
 * 求在该柱状图中，能够勾勒出来的矩形的最大面积。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 
 * 
 * 输入：heights = [2,1,5,6,2,3]
 * 输出：10
 * 解释：最大的矩形为图中红色区域，面积为 10
 * 
 * 
 * 示例 2：
 * 
 * 
 * 
 * 
 * 输入： heights = [2,4]
 * 输出： 4
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 0 
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        let mut left: Vec<_> = (0..len).collect();
        let mut stack = Vec::new();
        for i in 0..len{
            let mut limit = i;
            while !stack.is_empty() && heights[*stack.last().unwrap() as usize] >= heights[i] {
                limit = stack.pop().unwrap();
            } 
            limit = left[limit] as usize;
            left[i] = limit;
            stack.push(i);
        }
        let mut stack = Vec::new();
        let mut right:Vec<_> = (0..len).collect();
        for i in (0..len).rev(){
            let mut limit = i;
            while !stack.is_empty() && heights[*stack.last().unwrap() as usize] >= heights[i] {
                limit = stack.pop().unwrap();
            } 
            limit = right[limit] as usize;
            right[i] = limit;
            stack.push(i);
        }
        let mut res = i32::MIN;
        for i in 0..len{
            res = res.max(heights[i] * (right[i] - left[i] + 1) as i32);
        }   
        res
    }
}

// @lc code=end

