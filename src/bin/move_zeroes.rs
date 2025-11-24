use std::env;

pub fn move_zeroes(nums: &mut Vec<i32>) {
    /*
        283. Move Zeroes (Easy)
        https://leetcode.com/problems/move-zeroes/description/
    */
    
    if nums.len() <= 1 {
        return;
    }

    let mut left:usize = 0;
    let mut right:usize = 1;
    while (right < nums.len()) && (left < right) {
        if nums[right] != 0 && nums[left] == 0 {
            nums.swap(left, right);
            left += 1;
            right += 1;
        } else if nums[left] == 0 {
            right += 1;
        } else {
            left += 1;
            right += 1;
        }
    }
}


fn main() {
    let mut nums: Vec<i32>;

    let args : Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        nums = vec![0, 1, 0, 3, 12];
    } else {
        nums = args[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect()
    }

    println!("-> nums   : {nums:?}");
    move_zeroes(&mut nums);
    println!("=> result : {nums:?}");
}