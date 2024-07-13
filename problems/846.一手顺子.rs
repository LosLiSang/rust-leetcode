/*
 * @lc app=leetcode.cn id=846 lang=rust
 *
 * [846] 一手顺子
 *
 * https://leetcode.cn/problems/hand-of-straights/description/
 *
 * algorithms
 * Medium (57.27%)
 * Likes:    249
 * Dislikes: 0
 * Total Accepted:    43.3K
 * Total Submissions: 75.5K
 * Testcase Example:  '[1,2,3,6,2,3,4,7,8]\n3'
 *
 * Alice 手中有一把牌，她想要重新排列这些牌，分成若干组，使每一组的牌数都是 groupSize ，并且由 groupSize 张连续的牌组成。
 * 
 * 给你一个整数数组 hand 其中 hand[i] 是写在第 i 张牌上的数值。如果她可能重新排列这些牌，返回 true ；否则，返回 false
 * 。
 * 
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：hand = [1,2,3,6,2,3,4,7,8], groupSize = 3
 * 输出：true
 * 解释：Alice 手中的牌可以被重新排列为 [1,2,3]，[2,3,4]，[6,7,8]。
 * 
 * 示例 2：
 * 
 * 
 * 输入：hand = [1,2,3,4,5], groupSize = 4
 * 输出：false
 * 解释：Alice 手中的牌无法被重新排列成几个大小为 4 的组。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= hand.length <= 10^4
 * 0 <= hand[i] <= 10^9
 * 1 <= groupSize <= hand.length
 * 
 * 
 * 
 * 
 * 注意：此题目与 1296
 * 重复：https://leetcode-cn.com/problems/divide-array-in-sets-of-k-consecutive-numbers/
 * 
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let hand_size = hand.len();
        if hand_size % group_size as usize != 0 {
            return false;
        };
        let mut hand = hand;
        hand.sort();
        let mut map = HashMap::new();
        for &num in &hand{
            *map.entry(num).or_insert(0) += 1;
        }
        for &card in &hand{
            if let Some(&count) = map.get(&card){
                if count > 0 {
                    for i in 0..group_size{
                        let cur_card = i + card;
                        if let Some(cur_count) = map.get_mut(&cur_card){
                            if *cur_count <= 0{
                                return false;
                            } else{
                                *cur_count -= 1;
                            }
                        }else{
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}



// @lc code=end

