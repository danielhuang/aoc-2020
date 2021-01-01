use itertools::Itertools;
use std::collections::HashSet;

#[util::bench]
fn main() -> (i32, i32) {
	let input = include_str!("1.txt");
	let nums: Vec<i32> = input.lines().map(|x| x.trim().parse().unwrap()).collect();
	let nums_set: HashSet<_> = nums.iter().copied().collect();

	let mut p1 = 0;
	let mut p2 = 0;

	for (a, b) in nums.into_iter().tuple_combinations() {
		let c = 2020 - a - b;
		if a + b == 2020 {
			p1 = a * b;
		}
		if nums_set.contains(&c) {
			p2 = a * b * c;
		}
	}
	
	(p1, p2)
}
