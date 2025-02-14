/*
    1352. Product of the Last K Numbers
    https://leetcode.com/problems/product-of-the-last-k-numbers/description/?envType=daily-question&envId=2025-02-14
*/

use std::io;

struct ProductOfNumbers {
    nums: Vec<i32>,
    nums_product: Vec<i32>
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self {
            nums: Vec::new(),
            nums_product: Vec::new(),
        }
    }
    
    fn add(&mut self, num: i32) {
        self.nums.push(num);
        for i in 0..self.nums_product.len() {
            self.nums_product[i] *= num;
        }
        self.nums_product.push(num);
    }
    
    fn get_product(&self, k: i32) -> i32 {
        self.nums_product[self.nums_product.len()-k as usize]
    }
}

fn main() {
    let mut obj = ProductOfNumbers::new();
    obj.add(5);
    obj.add(2);
    obj.add(4);
    obj.add(10);
    println!("res : {}", obj.get_product(3));

}