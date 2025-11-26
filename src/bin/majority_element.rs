use std::env;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    /*
        169. Majority Element
        https://leetcode.com/problems/majority-element/description/
    */

    let (mut max, mut element) = (1, nums[0]);
    for &i in nums.iter().skip(1) {
        if i == element {
            max += 1 ;
        } else {
            max -= 1;
        }
        if max < 1 {
            element = i;
            max = 1;
        }
    }
    element
}

fn main() {
    let nums: Vec<i32>;

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        nums = vec![3, 2, 3];
    } else {
        nums = args[0].split(',').map(|x| x.parse::<i32>().unwrap()).collect()
    }

    println!("-> nums   : {nums:?}");
    let result = majority_element(nums);
    println!("=> result : {result:?}");
}