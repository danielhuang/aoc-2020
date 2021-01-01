#![feature(hash_drain_filter)]

use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

fn get_input() -> &'static str {
	include_str!("19.txt")
}

fn rule_to_regex(map: &HashMap<i64, String>, id: i64, p2: bool) -> String {
	let line = map[&id].to_string();
	if line.contains('"') {
		return line.chars().nth(1).unwrap().to_string();
	}
	if p2 && id == 8 {
		let subfinal = rule_to_regex(&map, 42, p2);
		return format!("({})+", subfinal);
	}
	if p2 && id == 11 {
		let subfinal1 = rule_to_regex(&map, 42, p2);
		let subfinal2 = rule_to_regex(&map, 31, p2);
		let mut final1 = vec![];
		for i in 1..5 {
			let mut final2 = "".to_string();
			for _ in 0..i {
				final2 += &subfinal1;
			}
			for _ in 0..i {
				final2 += &subfinal2;
			}
			final1.push(final2);
		}
		return format!("({})", final1.join("|"));
	}
	let subfinal: String = line
		.split_whitespace()
		.map(|x| {
			if let Ok(x) = x.parse::<i64>() {
				rule_to_regex(&map, x, p2)
			} else {
				x.to_string()
			}
		})
		.collect();
	match id {
		8 if p2 => format!("({})+", subfinal),
		11 if p2 => format!("({})+", subfinal),
		_ => format!("({})", subfinal),
	}
}

#[util::bench]
fn main() -> (usize, usize) {
	let input = get_input();

	let (rules, msgs): (&str, &str) = input.split("\n\n").collect_tuple().unwrap();
	let rules = rules.to_string();

	let mut map = HashMap::new();

	for line in rules.lines() {
		let (id, rule) = line.split(": ").collect_tuple().unwrap();
		let id: i64 = id.parse().unwrap();
		let rule = rule.to_string();

		map.insert(id, rule);
	}

	let r1 = rule_to_regex(&map, 0, false);
	let r2 = rule_to_regex(&map, 0, true);

	let re1 = Regex::new(&format!("^{}$", r1)).unwrap();
	let re2 = Regex::new(&format!("^{}$", r2)).unwrap();

	let p1 = msgs.lines().filter(|x| re1.is_match(x)).count();
	let p2 = msgs.lines().filter(|x| re2.is_match(x)).count();

	(p1, p2)
}
