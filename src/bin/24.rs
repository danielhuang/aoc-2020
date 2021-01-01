use std::collections::HashSet;

use aoc::Coordinate;
use defaultmap::DefaultHashMap;

fn get_input() -> Vec<Coordinate> {
	let input = include_str!("24.txt");
	let mut result = Vec::new();
	for line in input.lines() {
		let mut coord = Coordinate(0, 0);
		let mut moves = Vec::new();
		let mut i = 0;
		while i < line.len() {
			let c = line.chars().nth(i).unwrap();
			match c {
				'e' | 'w' => {
					moves.push(&line[i..(i + 1)]);
					i += 1;
				}
				's' | 'n' => {
					moves.push(&line[i..(i + 2)]);
					i += 2;
				}
				_ => unreachable!(),
			}
		}
		for mv in moves {
			let d = match mv {
				"e" => Coordinate(1, 0),
				"se" => Coordinate(1, -1),
				"sw" => Coordinate(0, -1),
				"w" => Coordinate(-1, 0),
				"nw" => Coordinate(-1, 1),
				"ne" => Coordinate(0, 1),
				_ => unreachable!(),
			};
			coord += d;
		}
		result.push(coord);
	}
	result
}

const ADJACENT: &[Coordinate] = &[
	Coordinate(1, 0),
	Coordinate(1, -1),
	Coordinate(0, -1),
	Coordinate(-1, 0),
	Coordinate(-1, 1),
	Coordinate(0, 1),
];

fn count_adjacent(grid: &DefaultHashMap<Coordinate, bool>, loc: Coordinate) -> usize {
	ADJACENT.iter().filter(|&&x| grid[loc + x]).count()
}

fn mutate(grid: &mut DefaultHashMap<Coordinate, bool>, prev: &DefaultHashMap<Coordinate, bool>) {
	let mut keys: HashSet<_> = grid.keys().copied().collect();
	for key in keys.clone() {
		for &a in ADJACENT {
			keys.insert(key + a);
		}
	}
	for &a in &keys {
		let adj = count_adjacent(&prev, a);
		if prev[a] && (adj == 0 || adj > 2) {
			grid[a] = false;
		}
		if !prev[a] && adj == 2 {
			grid[a] = true;
		}
	}
}

#[util::bench]
fn main() -> (usize, usize) {
	let input = get_input();
	let mut grid = DefaultHashMap::new(false);

	for &coord in &input {
		grid[coord] = !grid[coord];
	}

	let p1 = grid.values().filter(|&&x| x).count();

	for _ in 0..100 {
		let prev = grid.clone();
		mutate(&mut grid, &prev);
	}

	let p2 = grid.values().filter(|&&x| x).count();

	(p1, p2)
}
