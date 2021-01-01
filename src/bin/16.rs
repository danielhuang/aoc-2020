#![feature(hash_drain_filter)]

use std::{
	cell::RefCell,
	collections::{HashMap, HashSet},
};

use itertools::Itertools;

fn get_input() -> &'static str {
	include_str!("16.txt")
}

#[util::bench]
fn main() -> i64 {
	let input = get_input();

	let (defs, ticket, others) = input.split("\n\n").collect_tuple().unwrap();

	let defs: HashMap<&str, _> = defs
		.lines()
		.map(|x| {
			let (name, bounds) = x.split(": ").collect_tuple().unwrap();
			let (a, b) = bounds
				.split(" or ")
				.map(|x| {
					let (left, right) = x
						.split('-')
						.map(|x| x.parse::<i64>().unwrap())
						.collect_tuple()
						.unwrap();

					left..=right
				})
				.collect_tuple()
				.unwrap();

			(name, (a, b))
		})
		.collect();

	let ticket = ticket
		.lines()
		.nth(1)
		.unwrap()
		.split(',')
		.map(|x| x.parse::<i64>().unwrap())
		.collect_vec();

	let others: Vec<Vec<i64>> = others
		.lines()
		.skip(1)
		.map(|line| {
			line.split(',')
				.map(|x| x.parse::<i64>().unwrap())
				.collect_vec()
		})
		.collect_vec();

	let valid = others
		.iter()
		.filter(|ticket| {
			ticket.iter().all(|field| {
				defs.values()
					.any(|(a, b)| a.contains(field) || b.contains(field))
			})
		})
		.collect_vec();

	let all = defs.keys().copied().collect::<HashSet<_>>();

	let possibilities = vec![RefCell::new(all.clone()); defs.len()];

	for &possibility in &all {
		for index in 0..possibilities.len() {
			for ticket in &valid {
				let field = ticket[index];
				let (a, b) = defs.get(possibility).unwrap();
				if !(a.contains(&field) || b.contains(&field)) {
					possibilities[index].borrow_mut().remove(possibility);
				}
			}
		}
	}

	while possibilities.iter().any(|x| x.borrow().len() > 1) {
		for (p1, p2) in possibilities.iter().tuple_combinations() {
			if p1.borrow().is_superset(&p2.borrow()) {
				p1.borrow_mut().drain_filter(|x| p2.borrow().contains(x));
			}
			if p2.borrow().is_superset(&p1.borrow()) {
				p2.borrow_mut().drain_filter(|x| p1.borrow().contains(x));
			}
		}
	}

	let mut total = 1;

	for (i, possibility) in possibilities.iter().enumerate() {
		let possibility = possibility.borrow();
		let possibility = possibility.iter().next().unwrap();
		if possibility.starts_with("departure") {
			total *= ticket[i];
		}
	}

	total
}
