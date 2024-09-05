/*
 * @lc app=leetcode.cn id=363 lang=rust
 *
 * [363] 矩形区域不超过 K 的最大数值和
 *
 * https://leetcode.cn/problems/max-sum-of-rectangle-no-larger-than-k/description/
 *
 * algorithms
 * Hard (48.33%)
 * Likes:    489
 * Dislikes: 0
 * Total Accepted:    44.9K
 * Total Submissions: 92.9K
 * Testcase Example:  '[[1,0,1],[0,-2,3]]\n2'
 *
 * 给你一个 m x n 的矩阵 matrix 和一个整数 k ，找出并返回矩阵内部矩形区域的不超过 k 的最大数值和。
 * 
 * 题目数据保证总会存在一个数值和不超过 k 的矩形区域。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：matrix = [[1,0,1],[0,-2,3]], k = 2
 * 输出：2
 * 解释：蓝色边框圈出来的矩形区域 [[0, 1], [-2, 3]] 的数值和是 2，且 2 是不超过 k 的最大数字（k = 2）。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：matrix = [[2,2,-1]], k = 3
 * 输出：3
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * m == matrix.length
 * n == matrix[i].length
 * 1 
 * -100 
 * -10^5 
 * 
 * 
 * 
 * 
 * 进阶：如果行数远大于列数，该如何设计解决方案？
 * 
 */

// @lc code=start
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let row_len = matrix.len();
        let column_len = matrix[0].len();
        let mut prefix_sum = vec![vec![0; column_len + 1]; row_len + 1];

        for (i, row) in matrix.iter().enumerate(){
            let mut row_sum = 0;
            for (j, val) in row.iter().enumerate(){
                row_sum += val;
                prefix_sum[i+1][j+1] = prefix_sum[i][j+1] + row_sum;
            }
        } 
        // println!("{:?}", prefix_sum);
        let mut answer = std::i32::MIN; 
        for i_front in 1..row_len + 1{
            for i_back in 0..i_front{
                for j_front in 1..column_len + 1{
                    for j_back in 0..j_front{
                        let res = prefix_sum[i_front][j_front] + prefix_sum[i_back][j_back] - prefix_sum[i_front][j_back] - prefix_sum[i_back][j_front];
                        // println!("{:?}", res);
                        if  res <= k {
                            answer = answer.max(res);
                        }
                    }
                }
            }
        }       
        answer
    }
}
// @lc code=end

