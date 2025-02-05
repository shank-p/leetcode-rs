/*
    1790. Check if One String Swap Can Make Strings Equal
    https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/description/?envType=daily-question&envId=2025-02-05
*/

use std::io;

pub fn are_almost_equal(s1: String, s2: String) -> bool {
    let mut can_swap: bool = true;
    let mut diffs : Vec<char> = Vec::with_capacity(2);
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            continue;
        } else if can_swap {
            if diffs.len() == 0 {
                diffs.push(c1);
                diffs.push(c2);
            } else {
                let (c2_prev, c1_prev) = (diffs.pop().unwrap(), diffs.pop().unwrap());
                if (c1 != c2_prev) || (c2 != c1_prev) {
                    return false;
                }
                can_swap = false;
            }
        } else {
            return false;
        }
    }
    diffs.len() == 0
}

fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();

    io::stdin()
        .read_line(&mut s1)
        .expect("Unable to read input `s1`!");
    let s1: String = s1.trim().parse().unwrap();

    io::stdin()
        .read_line(&mut s2)
        .expect("Unable to read input `s2`!");
    let s2: String = s2.trim().parse().unwrap();

    println!("res : {:?}", are_almost_equal(s1, s2));
}