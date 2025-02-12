/*
    2342. Max Sum of a Pair With Equal Sum of Digits
    https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/description/?envType=daily-question&envId=2025-02-12
*/

use std::cmp::max;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;

pub fn digits_sum(mut num: i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        sum += num%10;
        num /= 10;
    }
    sum
}

pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut max_val_map: HashMap<i32, i32> = HashMap::new();
    
    let mut max_sum: i32 = -1;
    for i in 0..nums.len() {
        let sum = digits_sum(nums[i]);

        match max_val_map.entry(sum) {
            Entry::Occupied(mut entry) => {
                let val = entry.get_mut();
                max_sum = max(max_sum, *val+nums[i]);
                if *val < nums[i] {
                    *val = nums[i];
                }
            },
            Entry::Vacant(entry) => {
                entry.insert(nums[i]);
            }
        };
    }
    max_sum
}

fn main() {
    let mut nums = String::new();
    io::stdin()
        .read_line(&mut nums)
        .expect("Unable to read input `nums`!");
    let nums: Vec<i32> = nums.split_whitespace()
                                .map(|s| s.parse().unwrap())
                                .collect();
    
    println!("res : {:?}", maximum_sum(nums));
}