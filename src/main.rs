mod lib;

use lib::Solution;
fn main() {
    let nums = vec![1, 3, 2, 8, 4, 9];
    let res = Solution::max_profit(nums, 2);
    println!("{res}");

}
