/*
    2364. Count Number of Bad Pairs
    https://leetcode.com/problems/count-number-of-bad-pairs/description/?envType=daily-question&envId=2025-02-09
*/

use std::io;
use std::collections::HashMap;

pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    let total_pairs: i64  = (((nums.len())*(nums.len()-1))/2) as i64;
    let mut good_pairs: i64 = 0;
    let mut good_pairs_map: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        if let Some(freq) = good_pairs_map.get_mut(&((i as i32)-nums[i])) {
            good_pairs += *freq as i64;
            *freq += 1;
        } else {
            good_pairs_map.insert((i as i32)-nums[i], 1);
        }
    }
    total_pairs - good_pairs
}

fn main() {
    let mut nums = String::new();
    io::stdin()
        .read_line(&mut nums)
        .expect("Unable to read input `nums`!");
    let nums: Vec<i32> = nums.split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect();

    println!("res : {:?}", count_bad_pairs(nums));
}