/*
    1800. Maximum Ascending Subarray Sum
    https://leetcode.com/problems/maximum-ascending-subarray-sum/description/?envType=daily-question&envId=2025-02-04
*/

use std::io;
use std::cmp::max;

pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let nums_len = nums.len();
    if nums_len <= 1 {
        return nums_len as i32;
    }

    let (mut max_sum, mut cur_sum ) = (nums[0], nums[0]);
    for i in 1..nums_len {
        if nums[i-1] < nums[i] {
            cur_sum += nums[i];
        } else {
            cur_sum = nums[i];
        }
        max_sum = max(cur_sum, max_sum);
    }
    max_sum
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

    println!("res : {:?}", max_ascending_sum(nums));
}