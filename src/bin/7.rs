use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse(s: &str) -> (String, usize) {
	if s == "no other bags" {
		return ("".into(), 0);
	}

	let s = s.replace(" bags", "");
	let s = s.replace(" bag", "");

	let num = s.split(' ').next().unwrap().parse().unwrap();

	let (_, rest) = s.splitn(2, ' ').collect_tuple().unwrap();

	(rest.to_string(), num)
}

fn count(all_bags: &HashMap<String, Vec<(String, usize)>>, name: &str) -> usize {
	if name.is_empty() {
		return 0;
	}
	let inside = &all_bags[name];
	inside
		.iter()
		.map(|x| count(&all_bags, &x.0) * x.1)
		.sum::<usize>()
		+ 1
}

#[util::bench]
fn main() -> usize {
	let input = include_str!("7.txt");
	let texts: Vec<_> = input.split('\n').collect();

	let mut all_bags = HashMap::new();

	let mut available = HashSet::new();
	available.insert("shiny gold".to_string());

	for &text in &texts {
		let text = &text[0..(text.len() - 1)];
		let (outer, inner) = text.split(" contain ").collect_tuple().unwrap();
		let inner = inner.split(", ").map(parse).collect_vec();
		let outer = &outer[0..(outer.len() - 5)];
		all_bags.insert(outer.to_string(), inner);
	}

	count(&all_bags, "shiny gold") - 1
}
