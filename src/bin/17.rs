use std::collections::HashSet;

use aoc::Coordinate4D;
use defaultmap::DefaultHashMap;

fn get_input() -> DefaultHashMap<Coordinate4D, Item> {
	let input = include_str!("17.txt");
	parse_input(input)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Item {
	Active,
	Inactive,
}

fn adjacent() -> Vec<Coordinate4D> {
	let mut result = vec![];
	for i in -1..=1 {
		for j in -1..=1 {
			for k in -1..=1 {
				for l in -1..=1 {
					if i != 0 || j != 0 || k != 0 || l != 0 {
						result.push(Coordinate4D(i, j, k, l));
					}
				}
			}
		}
	}
	result
}

fn count_adjacent(
	grid: &DefaultHashMap<Coordinate4D, Item>,
	loc: Coordinate4D,
	target: Item,
) -> usize {
	adjacent()
		.into_iter()
		.filter(|&x| grid[loc + x] == target)
		.count()
}

fn parse_input(input: &str) -> DefaultHashMap<Coordinate4D, Item> {
	let mut i = 0;
	let mut j = 0;

	let mut result = DefaultHashMap::new(Item::Inactive);

	for c in input.chars() {
		if c == '\n' {
			j += 1;
			i = 0;
		} else {
			result[Coordinate4D(i, j, 0, 0)] = match c {
				'.' => Item::Inactive,
				'#' => Item::Active,
				_ => unimplemented!(),
			};
			i += 1;
		}
	}

	result
}

fn mutate(
	grid: &mut DefaultHashMap<Coordinate4D, Item>,
	prev: &DefaultHashMap<Coordinate4D, Item>,
) {
	let mut keys: HashSet<_> = grid.keys().copied().collect();
	for key in keys.clone() {
		for a in adjacent() {
			keys.insert(key + a);
		}
	}
	for &a in &keys {
		let adj = count_adjacent(&prev, a, Item::Active);
		if prev[a] == Item::Active && !(adj == 2 || adj == 3) {
			grid[a] = Item::Inactive;
		}
		if prev[a] == Item::Inactive && adj == 3 {
			grid[a] = Item::Active;
		}
	}
}

#[util::bench]
fn main() -> usize {
	let mut grid = get_input();

	for _ in 0..6 {
		let prev = grid.clone();
		mutate(&mut grid, &prev);
	}

	grid.values().filter(|&&x| x == Item::Active).count()
}
