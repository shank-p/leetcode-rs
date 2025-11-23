use std::{cmp::Ordering, env};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let nums_len = nums.len() as i32;
    let (mut left, mut right) = (0, nums_len-1);
    while left < right {
        let res = nums[left as usize] + nums[right as usize];
        match  res.cmp(&target) {
            Ordering::Equal => return vec![left+1, right+1],
            Ordering::Less => left+=1,
            Ordering::Greater => right-=1,
        }
    }
    vec![-1, -1]
}



fn main() {
    /*
        167. Two Sum II - Input Array Is Sorted
        https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/description/
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