/*
 * @lc app=leetcode.cn id=85 lang=rust
 *
 * [85] 最大矩形
 *
 * https://leetcode.cn/problems/maximal-rectangle/description/
 *
 * algorithms
 * Hard (55.19%)
 * Likes:    1645
 * Dislikes: 0
 * Total Accepted:    198.9K
 * Total Submissions: 360.4K
 * Testcase Example:  '[["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]'
 *
 * 给定一个仅包含 0 和 1 、大小为 rows x cols 的二维二进制矩阵，找出只包含 1 的最大矩形，并返回其面积。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：matrix =
 * [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
 * 输出：6
 * 解释：最大矩形如上图所示。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：matrix = [["0"]]
 * 输出：0
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：matrix = [["1"]]
 * 输出：1
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * rows == matrix.length
 * cols == matrix[0].length
 * 1 <= row, cols <= 200
 * matrix[i][j] 为 '0' 或 '1'
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let row = matrix.len();
        let column = matrix[0].len();
        let mut res = i32::MIN;

        let mut horizon = vec![vec![0; column + 1]; row + 1];
        for i in 1..=row {
            for j in 1..=column {
                if matrix[i-1][j-1] == '1' {
                    horizon[i][j] = horizon[i-1][j] + 1;
                }
            }
        }
        for i in 1..=row {
            let mut stack:Vec<usize> = Vec::new();
            let mut left:Vec<usize> = (0..=column).collect(); // left[j] 存以horizon[i][j]为最小值的最长子数组的最左端
            for j in 1..=column{
                let mut left_limit = j; // 表示以*stack.last().unwrap()为最小值的最左边的idx
                while !stack.is_empty() && horizon[i][*stack.last().unwrap()] >= horizon[i][j] { // 递增栈
                    left_limit = stack.pop().unwrap();
                }
                left[j] = left[left_limit];
                stack.push(j);
            }
            let mut right:Vec<usize> = (0..=column).collect(); // 存以horizon[i][j]为最小值的最长子数组的最右端
            let mut stack:Vec<usize> = Vec::new();
            for j in (1..=column).rev(){
                let mut right_limit = j; // 表示以*stack.last().unwrap()为最小值的最左边的idx
                while !stack.is_empty() && horizon[i][*stack.last().unwrap()] >= horizon[i][j] { // 递增栈
                    right_limit = stack.pop().unwrap();
                }
                right[j] = right[right_limit];
                stack.push(j);
            }
            for j in 1..=column{
                res = res.max((right[j] - left[j] + 1) as i32 * horizon[i][j]);
            }
        }
        res
    }
}

// @lc code=end

