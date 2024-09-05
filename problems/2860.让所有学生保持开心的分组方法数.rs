/*
 * @lc app=leetcode.cn id=2860 lang=rust
 *
 * [2860] 让所有学生保持开心的分组方法数
 *
 * https://leetcode.cn/problems/happy-students/description/
 *
 * algorithms
 * Medium (56.19%)
 * Likes:    50
 * Dislikes: 0
 * Total Accepted:    13.3K
 * Total Submissions: 20.4K
 * Testcase Example:  '[1,1]'
 *
 * 给你一个下标从 0 开始、长度为 n 的整数数组 nums ，其中 n 是班级中学生的总数。班主任希望能够在让所有学生保持开心的情况下选出一组学生：
 * 
 * 如果能够满足下述两个条件之一，则认为第 i 位学生将会保持开心：
 * 
 * 
 * 这位学生被选中，并且被选中的学生人数 严格大于 nums[i] 。
 * 这位学生没有被选中，并且被选中的学生人数 严格小于 nums[i] 。
 * 
 * 
 * 返回能够满足让所有学生保持开心的分组方法的数目。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,1]
 * 输出：2
 * 解释：
 * 有两种可行的方法：
 * 班主任没有选中学生。
 * 班主任选中所有学生形成一组。 
 * 如果班主任仅选中一个学生来完成分组，那么两个学生都无法保持开心。因此，仅存在两种可行的方法。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [6,0,3,3,6,7,2,7]
 * 输出：3
 * 解释：
 * 存在三种可行的方法：
 * 班主任选中下标为 1 的学生形成一组。
 * 班主任选中下标为 1、2、3、6 的学生形成一组。
 * 班主任选中所有学生形成一组。 
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 10^5
 * 0 <= nums[i] < nums.length
 * 
 * 
 */

// @lc code=start
use std::{cmp::Reverse, collections::{btree_map::Entry, BTreeMap, BinaryHeap, HashMap}};
impl Solution {
    pub fn count_ways(nums: Vec<i32>) -> i32 {
        let mut map = BTreeMap::new();
        for num in nums{
            *map.entry(num).or_insert(0) += 1;
        }
        
        let mut answer = 0;
        let entrys: Vec<_> = map.iter().collect();
        let mut cnt = 0;
        let mut flag = true;
        for (&k, &v) in entrys{ 
            if k > cnt && flag{ 
                // println!("{:?} {:?}", k, cnt);
                answer += 1;
            } 
            cnt += v;
            flag = k < cnt;
        }
        if *map.last_key_value().unwrap().0 < cnt{ // 全选
            answer += 1;
        } 
        answer
    }
}
// @lc code=end

