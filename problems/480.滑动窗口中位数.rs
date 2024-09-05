/*
 * @lc app=leetcode.cn id=480 lang=rust
 *
 * [480] 滑动窗口中位数
 *
 * https://leetcode.cn/problems/sliding-window-median/description/
 *
 * algorithms
 * Hard (41.97%)
 * Likes:    474
 * Dislikes: 0
 * Total Accepted:    45.2K
 * Total Submissions: 108.5K
 * Testcase Example:  '[1,3,-1,-3,5,3,6,7]\n3'
 *
 * 中位数是有序序列最中间的那个数。如果序列的长度是偶数，则没有最中间的数；此时中位数是最中间的两个数的平均数。
 * 
 * 例如：
 * 
 * 
 * [2,3,4]，中位数是 3
 * [2,3]，中位数是 (2 + 3) / 2 = 2.5
 * 
 * 
 * 给你一个数组 nums，有一个长度为 k 的窗口从最左端滑动到最右端。窗口中有 k 个数，每次窗口向右移动 1
 * 位。你的任务是找出每次窗口移动后得到的新窗口中元素的中位数，并输出由它们组成的数组。
 * 
 * 
 * 
 * 示例：
 * 
 * 给出 nums = [1,3,-1,-3,5,3,6,7]，以及 k = 3。
 * 
 * 
 * 窗口位置                      中位数
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       1
 * ⁠1 [3  -1  -3] 5  3  6  7      -1
 * ⁠1  3 [-1  -3  5] 3  6  7      -1
 * ⁠1  3  -1 [-3  5  3] 6  7       3
 * ⁠1  3  -1  -3 [5  3  6] 7       5
 * ⁠1  3  -1  -3  5 [3  6  7]      6
 * 
 * 
 * 因此，返回该滑动窗口的中位数数组 [1,-1,-1,3,5,6]。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 你可以假设 k 始终有效，即：k 始终小于等于输入的非空数组的元素个数。
 * 与真实值误差在 10 ^ -5 以内的答案将被视作正确答案。
 * 
 * 
 */

// @lc code=start

use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};
impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        // 如果k为奇数: 保证小根堆里数组的个数比大根堆的个数多一个
        let mut big_heap = BinaryHeap::new();
        let mut small_heap = BinaryHeap::new(); // 最前面的是最小值
        let (mut left, mut right): (usize, usize) = (0, 0);
        let mut balance = 0; // big_heap - small_heap  小 - 大
        let mut map = HashMap::new(); 
        let mut answer = Vec::new();
        let k = k as usize;
        while right < k{
            small_heap.push(Reverse(nums[right]));
            right += 1;
        }
        for _ in 0..right/2{
            big_heap.push(small_heap.pop().unwrap().0);
        }
        
        while right < nums.len(){
            balance = 0;
            if k % 2 == 0{
                answer.push((small_heap.peek().unwrap().0 as i64 + *big_heap.peek().unwrap() as i64 ) as f64  / 2.0);
            } else {
                answer.push(small_heap.peek().unwrap().0 as f64);
            }
            if nums[left] < small_heap.peek().unwrap().0{ // 说明要删除的数在big_heap
                balance -= 1;
            }else{
                balance += 1; 
            }
            *map.entry(nums[left]).or_insert(0) += 1; 
            
            if nums[right] < small_heap.peek().unwrap().0{ // 这个数比较小, 要插入big_heap
                big_heap.push(nums[right]);
                balance += 1;
            } else{
                small_heap.push(Reverse(nums[right]) );
                balance -= 1;
            }
            if balance > 0 { // big_heap 数比较多
                small_heap.push(Reverse(big_heap.pop().unwrap()));
            }else if balance < 0 {
                big_heap.push(small_heap.pop().unwrap().0);
            }
            while small_heap.capacity() != 0 && *map.get(&small_heap.peek().unwrap().0).unwrap_or(&0) > 0 {
                *map.get_mut(&small_heap.peek().unwrap().0).unwrap() -= 1;
                small_heap.pop();
            }
            while big_heap.peek() != None && *map.get(&big_heap.peek().unwrap()).unwrap_or(&0) > 0 {
                *map.get_mut(&big_heap.peek().unwrap()).unwrap() -= 1;
                big_heap.pop();
            }
            
            
            right += 1;
            left += 1;
        }
        if k % 2 == 0{
                answer.push((small_heap.peek().unwrap().0 as i64 + *big_heap.peek().unwrap() as i64 ) as f64  / 2.0);
            } else {
                answer.push(small_heap.peek().unwrap().0 as f64);
            }
           
        answer 
    }
}
// @lc code=end

