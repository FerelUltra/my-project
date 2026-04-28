use std::cmp;
fn main(){}

pub fn max_profit(prices: Vec<i32>) -> i32{
	let mut left = 0;
	let mut max = 0;
	for right in 0..prices.len(){
		if prices[right] < prices[left]{
			left = right
		}

		max = cmp::max(max, prices[right] - prices[left]);
	}

	max
}