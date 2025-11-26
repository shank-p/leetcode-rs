use std::env;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    /*
        26. Remove Duplicates from Sorted Array
        https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
    */
    if nums.len() <= 1 {
        return nums.len() as i32;
    }
    let (mut left, mut right) = (0 as usize, 0 as usize);
    while (left <= right) && (right < nums.len()) {
        if left == right {
            right += 1;
        } 
        if nums[right] != nums[left] {
            left += 1;
            nums[left] = nums[right];
            right += 1;
        } else {
            right += 1;
        }
    }
    return  (left+1) as i32;

}


fn main() {
    let mut nums: Vec<i32>;

    let args : Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        nums = vec![0,0,1,1,1,2,2,3,3,4];
    } else {
        nums = args[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect()
    }

    println!("-> nums   : {nums:?}");
    let offset = remove_duplicates(&mut nums);
    println!("=> result : {nums:?}");
    println!("=> result : {offset:?}");
}