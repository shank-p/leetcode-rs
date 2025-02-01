/*
    3151. Special Array I
    https://leetcode.com/problems/special-array-i/description/?envType=daily-question&envId=2025-02-01
*/

use std::io;

pub fn is_array_special(nums: Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return true;
    }
    let mut prev_parity_is_even: bool = nums[0]%2 == 0;
    for i in nums[1..].iter() {
        let curr_parity_is_even = i%2 == 0;
        if curr_parity_is_even == prev_parity_is_even {
            return false
        }
        prev_parity_is_even = !prev_parity_is_even;
    }
    true
}

fn main() {
    let mut nums = String::new();
    io::stdin()
        .read_line(&mut nums)
        .expect("Unable to read input `nums`!");

    let nums: Vec<i32> = nums.split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

    println!("res : {:?}", is_array_special(nums));
}