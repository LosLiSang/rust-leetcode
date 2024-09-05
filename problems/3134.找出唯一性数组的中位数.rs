/*
 * @lc app=leetcode.cn id=3134 lang=rust
 *
 * [3134] 找出唯一性数组的中位数
 *
 * https://leetcode.cn/problems/find-the-median-of-the-uniqueness-array/description/
 *
 * algorithms
 * Hard (43.09%)
 * Likes:    30
 * Dislikes: 0
 * Total Accepted:    6.1K
 * Total Submissions: 12.1K
 * Testcase Example:  '[1,2,3]'
 *
 * 给你一个整数数组 nums 。数组 nums 的 唯一性数组 是一个按元素从小到大排序的数组，包含了 nums 的所有非空子数组中不同元素的个数。
 * 
 * 换句话说，这是由所有 0 <= i <= j < nums.length 的 distinct(nums[i..j]) 组成的递增数组。
 * 
 * 其中，distinct(nums[i..j]) 表示从下标 i 到下标 j 的子数组中不同元素的数量。
 * 
 * 返回 nums 唯一性数组 的 中位数 。
 * 
 * 注意，数组的 中位数 定义为有序数组的中间元素。如果有两个中间元素，则取值较小的那个。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,2,3]
 * 
 * 输出：1
 * 
 * 解释：
 * 
 * nums 的唯一性数组为 [distinct(nums[0..0]), distinct(nums[1..1]),
 * distinct(nums[2..2]), distinct(nums[0..1]), distinct(nums[1..2]),
 * distinct(nums[0..2])]，即 [1, 1, 1, 2, 2, 3] 。唯一性数组的中位数为 1 ，因此答案是 1 。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [3,4,3,4,5]
 * 
 * 输出：2
 * 
 * 解释：
 * 
 * nums 的唯一性数组为 [1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3] 。唯一性数组的中位数为 2
 * ，因此答案是 2 。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：nums = [4,3,5,4]
 * 
 * 输出：2
 * 
 * 解释：
 * 
 * nums 的唯一性数组为 [1, 1, 1, 1, 2, 2, 2, 3, 3, 3] 。唯一性数组的中位数为 2 ，因此答案是 2 。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 10^5
 * 1 <= nums[i] <= 10^5
 * 
 * 
 */

// @lc code=start
use std::collections::{hash_map, HashMap};



impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let median = (len * (len + 1) / 2 + 1) / 2; 
        let mut low = 1;
        let mut high = len;
        
        // 检查有多少个不同元素个数少于t的子数组的个数是否小于median
        fn check(nums: &[i32], t: usize, median: usize) -> bool{
            let mut cnt = HashMap::new(); // 子数组中不同元素的个数
            let mut j = 0;
            let mut total = 0; // 这样的子数组的个数
            for (i, &key) in nums.iter().enumerate(){
                *cnt.entry(key).or_insert(0) += 1;
                while cnt.len() >  t{
                    *cnt.entry(nums[j]).or_insert(0) -= 1;
                    if *cnt.get(&nums[j]).unwrap_or(&0) == 0{
                        cnt.remove(&nums[j]);
                    }
                    j += 1;
                }
                total += i - j + 1;
            }
            total >= median
        }
        let mut res = 0;
        while low  <= high{
            let mid = (low + high) / 2;
            if check(&nums, mid, median) {
                res = mid;
                high = mid - 1;
            }else {
                low = mid + 1;
            }
        }
        res as i32

    }
}
// @lc code=end
