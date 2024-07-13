/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 *
 * https://leetcode.cn/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (59.87%)
 * Likes:    2849
 * Dislikes: 0
 * Total Accepted:    902.8K
 * Total Submissions: 1.5M
 * Testcase Example:  '"23"'
 *
 * 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。
 * 
 * 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：digits = "23"
 * 输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：digits = ""
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：digits = "2"
 * 输出：["a","b","c"]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= digits.length <= 4
 * digits[i] 是范围 ['2', '9'] 的一个数字。
 * 
 * 
 */

// @lc code=start
impl Solution {   
    pub fn dfs(digits_list: &Vec<char>, res: &mut Vec<String>, str: String){
        let len = digits_list.len();
        let depth = str.len();
        if depth == len {
            res.push(str);
            return;
        }
        let cur = digits_list[depth];
        match cur {
            '2' => {
                let mut str_copy = str.clone();
                str_copy.push('a');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('b');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('c');
                Self::dfs(digits_list, res, str_copy);
            },
            '3' => {
                let mut str_copy = str.clone();
                str_copy.push('d');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('e');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('f');
                Self::dfs(digits_list, res, str_copy);
            },
            '4' => {
                let mut str_copy = str.clone();
                str_copy.push('h');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('i');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('g');
                Self::dfs(digits_list, res, str_copy);
            },
            '5' => {
                let mut str_copy = str.clone();
                str_copy.push('j');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('k');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('l');
                Self::dfs(digits_list, res, str_copy);
            },
            '6' => {
                let mut str_copy = str.clone();
                str_copy.push('m');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('n');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('o');
                Self::dfs(digits_list, res, str_copy);
            },
            '7' => {
                let mut str_copy = str.clone();
                str_copy.push('p');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('q');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('r');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('s');
                Self::dfs(digits_list, res, str_copy);
            },
            '8' => {
                let mut str_copy = str.clone();
                str_copy.push('t');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('u');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('v');
                Self::dfs(digits_list, res, str_copy);
            },
            '9' => {
                let mut str_copy = str.clone();
                str_copy.push('w');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('x');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('y');
                Self::dfs(digits_list, res, str_copy);
                let mut str_copy = str.clone();
                str_copy.push('z');
                Self::dfs(digits_list, res, str_copy);
            },
            _  => print!(""),
        } 
    }
    
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = Vec::new();
        let digits_list: Vec<char> = digits.chars().collect();
        if digits_list.is_empty(){

        }else{
            Self::dfs(&digits_list, &mut res, "".to_string());
        }
        res
    }
}
// @lc code=end

