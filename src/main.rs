mod lib;


use lib::Solution;




fn main() {
    let mut a = Some(Box::new(5));
    let b = a.as_mut();
    // *b.unwrap() = Box::new(6);
    let c = b.unwrap();
    **c = 1;
    println!("{:?}", a.unwrap());
    // let res = Solution::
    //     add_digits(38);
    // println!("{:?}", res);
}