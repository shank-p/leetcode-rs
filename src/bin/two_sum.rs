use std::{collections::HashMap, env};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&value) = map.get(&(num)) {
            return vec![value, i as i32];
        } else {
            map.insert(target-num, i as i32);
        }
    }
    vec![-1, -1]
}



fn main() {
    /*
        1. Two Sum [Easy]
        https://leetcode.com/problems/two-sum/description/
    */
    let nums: Vec<i32>;
    let target: i32;

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        nums = vec![2, 7, 11, 15];
        target = 9;
    } else {
        nums = args[0].split_whitespace().map(|x| x.parse().unwrap()).collect();
        target = args[1].parse().unwrap();
    }

    println!("-> nums   : {nums:?}");
    println!("-> target : {target}");
    let result = two_sum(nums, target);
    println!("=> result : {result:?}");
}