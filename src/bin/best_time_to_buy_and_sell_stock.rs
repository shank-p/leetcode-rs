use std::{cmp::max, env};

pub fn max_profit(prices: Vec<i32>) -> i32 {
    /*
        121. Best Time to Buy and Sell Stock
        https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
    */       
    let (mut max_pr, mut cur_pr) = (0, 0);
    let mut buy_idx: usize = 0;
    for sell_idx in 1..prices.len() {
        cur_pr = prices[sell_idx] - prices[buy_idx]; 
        max_pr = max(max_pr, cur_pr);
        if cur_pr <= 0 {
            cur_pr = 0;
            buy_idx = sell_idx;
        }
    }
    max_pr
}


fn main() {
    let prices: Vec<i32>;

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        prices = vec![7,1,5,3,6,4];
    } else {
        prices = args[0].split(',').map(|x| x.parse().unwrap()).collect();
    }

    println!("-> prices : {prices:?}");
    let result = max_profit(prices);
    println!("-> profit : {result}");
}