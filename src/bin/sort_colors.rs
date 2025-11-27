use std::{collections::HashMap, env::args};

pub fn sort_colors(nums: &mut Vec<i32>) {
   /*
        75. Sort Colors
        https://leetcode.com/problems/sort-colors/description/
    */

    // iteration-1 O(n) & O(n)
    // let mut maps: HashMap<i32, usize> = HashMap::new();
    // for i in nums.into_iter() {
    //     if let Some(key) = maps.get_mut(i) {
    //         *key+=1;
    //     } else {
    //         maps.insert(*i, 1);
    //     }
    // }
    // *nums = vec![0; maps.get(&0).cloned().unwrap_or(0)];
    // nums.extend(vec![1; maps.get(&1).cloned().unwrap_or(0)]);
    // nums.extend(vec![2; maps.get(&2).cloned().unwrap_or(0)]);

    // iteration-2 O(n) O(1)
    let (mut low, mut mid, mut high): (usize, usize, usize) = (0, 0, nums.len());
    while mid < high {
        match nums[mid] {
            0 => {
                nums.swap(low, mid);
                low+=1;
                mid+=1;
            },
            1 => {
                mid+=1;
            },
            _ => {
                high-=1;
                nums.swap(mid, high);
            }
        }
    }
}

fn main() {
    let mut nums: Vec<i32>;

    let args: Vec<String> = args().skip(1).collect();
    if args.len() != 1 {
        nums = vec![2,0,2,1,1,0];
    } else {
        nums = args[0].split(',').map(|x| x.parse().unwrap()).collect();
    }

    println!("-> nums : {nums:?}");
    sort_colors(&mut nums);
    println!("-> res  : {nums:?}");
}