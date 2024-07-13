/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 *
 * https://leetcode.cn/problems/4sum/description/
 *
 * algorithms
 * Medium (36.61%)
 * Likes:    1925
 * Dislikes: 0
 * Total Accepted:    598.4K
 * Total Submissions: 1.6M
 * Testcase Example:  '[1,0,-1,0,-2,2]\n0'
 *
 * 给你一个由 n 个整数组成的数组 nums ，和一个目标值 target 。请你找出并返回满足下述全部条件且不重复的四元组 [nums[a],
 * nums[b], nums[c], nums[d]] （若两个四元组元素一一对应，则认为两个四元组重复）：
 *
 *
 * 0 <= a, b, c, d < n
 * a、b、c 和 d 互不相同
 * nums[a] + nums[b] + nums[c] + nums[d] == target
 *
 *
 * 你可以按 任意顺序 返回答案 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,0,-1,0,-2,2], target = 0
 * 输出：[[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [2,2,2,2,2], target = 8
 * 输出：[[2,2,2,2]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 200
 * -10^9 <= nums[i] <= 10^9
 * -10^9 <= target <= 10^9
 *
 *
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut map = HashMap::new();
        for &num in &nums {
            *map.entry(num).or_insert(0) += 1;
        }
        nums.sort();
        let mut res = Vec::new();
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let temp1 = match nums[i].checked_add(nums[j]) {
                    Some(cal) => cal,
                    None => continue,
                };

                let (mut k, mut l) = (j + 1, nums.len() - 1);
                while k < l {
                    let temp2 = match nums[k].checked_add(nums[l]) {
                        Some(cal) => cal,

                        None => {
                            let l_value = nums[l];
                            while l_value == nums[l] && k < l {
                                l -= 1;
                            }
                            continue;
                        }
                    };
                    let res_value = match temp1.checked_add(temp2) {
                        Some(cal) => cal,
                        None => {
                            let l_value = nums[l];
                            while l_value == nums[l] && k < l {
                                l -= 1;
                            }
                            continue;
                        }
                    };
                    if res_value == target {
                        res.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                        let k_value = nums[k];
                        while k_value == nums[k] && k < l {
                            k += 1;
                        }
                        let l_value = nums[l];
                        while l_value == nums[l] && k < l {
                            l -= 1;
                        }
                    } else if res_value > target {
                        let l_value = nums[l];
                        while l_value == nums[l] && k < l {
                            l -= 1;
                        }
                    } else {
                        let k_value = nums[k];
                        while k_value == nums[k] && k < l {
                            k += 1;
                        }
                    }
                }
            }
        }

        res
    }
}

// @lc code=end
