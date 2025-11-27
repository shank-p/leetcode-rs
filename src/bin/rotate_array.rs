use std::env::args;

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    /*
        189. Rotate Array (Medium)
        https://leetcode.com/problems/rotate-array/description/
    */

    let rotates = k as usize % nums.len();
    // iteration-1 O(1) O(n)
    // let rotated = nums[..nums.len()-rotates].to_vec() ;
    // *nums = nums[nums.len()-rotates..].to_vec();
    // nums.extend(rotated);

    // iter-2 O(n) O(1)
    nums.reverse();
    nums[..rotates].reverse();
    nums[rotates..].reverse();

}


fn main() {
    let mut nums: Vec<i32>;
    let k: i32;

    let args: Vec<String> = args().skip(1).collect();
    if args.len() != 2 {
        nums = vec![1,2,3,4,5,6,7];
        k = 3;
    } else {
        nums = args[0].split(',').map(|x| x.parse().unwrap()).collect();
        k = args[1].parse().unwrap();
    }

    println!("-> nums : {nums:?}");
    println!("-> k    : {k}");
    rotate(&mut nums, k);
    println!("-> res  : {nums:?}");
}