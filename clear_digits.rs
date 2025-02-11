/*
    3174. Clear Digits
    https://leetcode.com/problems/clear-digits/description/?envType=daily-question&envId=2025-02-10
*/

use std::io;

pub fn clear_digits(s: String) -> String {
    let mut res_s: Vec<String> = Vec::new();
    for c in s.chars() {
        if c.is_numeric() {
            res_s.pop();
        } else {
            res_s.push(c.to_string());
        }
    }
    res_s.join("")
}

fn main() {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Unable to read input `s`!");

    let s = s.trim().to_string();
    println!("res : {:?}", clear_digits(s));
}