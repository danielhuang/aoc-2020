use itertools::Itertools;

fn get_num() -> i64 {
	let input = include_str!("9.txt");
	let nums: Vec<_> = input
		.split('\n')
		.map(|x| x.parse::<i64>().unwrap())
		.collect();

	let preamble_len = 25;

	for (i, &num) in nums.iter().enumerate().skip(preamble_len) {
		let before = nums.iter().take(i).rev().take(preamble_len).collect_vec();
		let before_pairs: Vec<(_, _)> = before.into_iter().tuple_combinations().collect();
		if !before_pairs.iter().any(|&(a, b)| a + b == num) {
			return num;
		}
	}

	unreachable!()
}

#[util::bench]
fn main() -> (i64, i64) {
	let num = get_num();
	let input = include_str!("9.txt");
	let nums: Vec<_> = input
		.split('\n')
		.map(|x| x.parse::<i64>().unwrap())
		.collect();

	let mut p2 = 0;

	let index_pairs: Vec<(_, _)> = (0..nums.len()).tuple_combinations().collect();
	for (a, b) in index_pairs {
		let subslice = &nums[a..b];
		let sum: i64 = subslice.iter().sum();
		if sum == num {
			p2 = subslice.iter().min().unwrap() + subslice.iter().max().unwrap();
		}
	}

	(num, p2)
}
