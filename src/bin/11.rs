use std::unimplemented;

use aoc::Coordinate;
use defaultmap::DefaultHashMap;

fn get_input() -> DefaultHashMap<Coordinate, Item> {
	let input = include_str!("11.txt");
	parse_input(input)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Item {
	Nothing,
	Floor,
	EmptySeat,
	FullSeat,
}

const ADJACENT: &[Coordinate] = &[
	Coordinate(0, 1),
	Coordinate(0, -1),
	Coordinate(1, 0),
	Coordinate(-1, 0),
	Coordinate(-1, -1),
	Coordinate(-1, 1),
	Coordinate(1, -1),
	Coordinate(1, 1),
];

fn count_adjacent(grid: &DefaultHashMap<Coordinate, Item>, loc: Coordinate, target: Item) -> usize {
	let mut count = 0;
	for &offset in ADJACENT {
		let mut cur = loc + offset;
		while grid[cur] == Item::Floor {
			cur += offset;
		}
		if grid[cur] == target {
			count += 1;
		}
	}
	count
}

fn parse_input(input: &str) -> DefaultHashMap<Coordinate, Item> {
	let mut i = 0;
	let mut j = 0;

	let mut result = DefaultHashMap::new(Item::Nothing);

	for c in input.chars() {
		if c == '\n' {
			j += 1;
			i = 0;
		} else {
			result[Coordinate(i, j)] = match c {
				'.' => Item::Floor,
				'#' => Item::FullSeat,
				'L' => Item::EmptySeat,
				_ => unimplemented!(),
			};
			i += 1;
		}
	}

	result
}

fn mutate(grid: &mut DefaultHashMap<Coordinate, Item>, prev: &DefaultHashMap<Coordinate, Item>) {
	for (&a, b) in grid.iter_mut() {
		if prev[a] == Item::EmptySeat && count_adjacent(&prev, a, Item::FullSeat) == 0 {
			*b = Item::FullSeat;
		}
		if prev[a] == Item::FullSeat && count_adjacent(&prev, a, Item::FullSeat) >= 5 {
			*b = Item::EmptySeat;
		}
	}
}

#[util::bench]
fn main() -> usize {
	let mut grid = get_input();

	let mut last = grid.clone();

	loop {
		let prev = grid.clone();
		mutate(&mut grid, &prev.clone());

		if grid == last {
			return grid.values().filter(|&&x| x == Item::FullSeat).count();
		}
		last = grid.clone();
	}
}
