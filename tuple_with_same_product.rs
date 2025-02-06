/*
    1726. Tuple with Same Product
    https://leetcode.com/problems/tuple-with-same-product/description/?envType=daily-question&envId=2025-02-06
*/

use std::io;
use std::collections::HashMap;

pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
    let mut pairs_count: HashMap<i32, i32> = HashMap::new();
    let mut product;
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            product = nums[i] * nums[j];
            *pairs_count.entry(product).or_insert(0) += 1;
        }
    }

    let mut total_pair_combinations = 0;
    for (&key, &val) in &pairs_count {
        if val >= 2 {
            total_pair_combinations += (val*(val-1))/2;
        }
    }

    total_pair_combinations*8
}

fn main() {
    let mut nums = String::new();
    io::stdin()
        .read_line(&mut nums)
        .expect("Unable to read input `nums`!");
    let nums: Vec<i32> = nums.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

    println!("res : {:?}", tuple_same_product(nums));
}