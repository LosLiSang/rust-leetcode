
mod lib;


use lib::Solution;
fn main() {
    // let s = String::from(")()())()()(");
    // let s1 = "intention".to_string();
    // let s2 = "execution".to_string();
    let nums = vec![1,0,-1,0,-2,2];
    
    // let nums1_vec = vec![4, 1, 2]; // 使用 Vec 定义 nums1
    // let nums2_vec = vec![1,3,4,2]; // 使用 Vec 定义 nums2
    // let matrix: Vec<Vec<char>> = vec![
    //     vec!['1', '0', '0', '0', '1'],
    //     vec!['1', '1', '0', '1', '1'],
    //     vec!['1', '1', '1', '1', '1'],
    // ];
    let res = Solution::four_sum(nums, 0);
    println!("{:?}", res);
}