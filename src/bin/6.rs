#![feature(iterator_fold_self)]

use std::collections::HashSet;

#[util::bench]
fn main() -> (usize, usize) {
	let input = include_str!("6.txt");
	let texts: Vec<_> = input.split("\n\n").collect();

	let p1 = texts
		.iter()
		.copied()
		.map(|x| {
			x.lines()
				.map(|x| x.chars().collect::<HashSet<_>>())
				.fold_first(|a, b| a.union(&b).copied().collect())
				.unwrap()
				.len()
		})
		.sum();

	let p2 = texts
		.iter()
		.copied()
		.map(|x| {
			x.lines()
				.map(|x| x.chars().collect::<HashSet<_>>())
				.fold_first(|a, b| a.intersection(&b).copied().collect())
				.unwrap()
				.len()
		})
		.sum();

	(p1, p2)
}
