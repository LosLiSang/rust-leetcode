/*
 * @lc app=leetcode.cn id=2095 lang=rust
 *
 * [2095] 删除链表的中间节点
 *
 * https://leetcode.cn/problems/delete-the-middle-node-of-a-linked-list/description/
 *
 * algorithms
 * Medium (58.42%)
 * Likes:    79
 * Dislikes: 0
 * Total Accepted:    38.1K
 * Total Submissions: 64.8K
 * Testcase Example:  '[1,3,4,7,1,2,6]'
 *
 * 给你一个链表的头节点 head 。删除 链表的 中间节点 ，并返回修改后的链表的头节点 head 。
 *
 * 长度为 n 链表的中间节点是从头数起第 ⌊n / 2⌋ 个节点（下标从 0 开始），其中 ⌊x⌋ 表示小于或等于 x 的最大整数。
 *
 *
 * 对于 n = 1、2、3、4 和 5 的情况，中间节点的下标分别是 0、1、1、2 和 2 。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：head = [1,3,4,7,1,2,6]
 * 输出：[1,3,4,1,2,6]
 * 解释：
 * 上图表示给出的链表。节点的下标分别标注在每个节点的下方。
 * 由于 n = 7 ，值为 7 的节点 3 是中间节点，用红色标注。
 * 返回结果为移除节点后的新链表。
 *
 *
 * 示例 2：
 *
 *
 *
 *
 * 输入：head = [1,2,3,4]
 * 输出：[1,2,4]
 * 解释：
 * 上图表示给出的链表。
 * 对于 n = 4 ，值为 3 的节点 2 是中间节点，用红色标注。
 *
 *
 * 示例 3：
 *
 *
 *
 *
 * 输入：head = [2,1]
 * 输出：[2]
 * 解释：
 * 上图表示给出的链表。
 * 对于 n = 2 ，值为 1 的节点 1 是中间节点，用红色标注。
 * 值为 2 的节点 0 是移除节点 1 后剩下的唯一一个节点。
 *
 *
 *
 * 提示：
 *
 *
 * 链表中节点的数目在范围 [1, 10^5] 内
 * 1 <= Node.val <= 10^5
 *
 *
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head.clone();
    let mut itr = &head;
            let mut cnt = 0;
    while !itr.is_none() {
        cnt += 1;
        itr = &itr.as_ref().unwrap().next;
    }
    cnt /= 2;
    cnt -= 1;
    let mut itr = &mut head;

    while cnt > 0 {
        cnt -= 1;
        itr = &mut itr.as_mut().unwrap().next;
    }
    itr.as_mut().unwrap().next = itr.as_mut().unwrap().next.take().unwrap().next;
    head
}
// @lc code=end
