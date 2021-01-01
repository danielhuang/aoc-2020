#![feature(str_split_once)]

use std::collections::{HashMap, HashSet};

use aoc::Coordinate;
use array2d::Array2D;
use defaultmap::DefaultHashMap;
use itertools::Itertools;
use multimap::MultiMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Item {
	Empty,
	Filled,
}

// Represents the dihedral group D4
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Transformation {
	None,
	R1,
	R2,
	R3,
	F1,
	F2,
	F3,
	F4,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Id(Transformation, i64);

fn all_transforms() -> Vec<Transformation> {
	vec![
		Transformation::None,
		Transformation::R1,
		Transformation::R2,
		Transformation::R3,
		Transformation::F1,
		Transformation::F2,
		Transformation::F3,
		Transformation::F4,
	]
}

fn transform(x: &Array2D<Item>, t: Transformation) -> Array2D<Item> {
	match t {
		Transformation::None => x.clone(),
		Transformation::R1 => {
			let n = x.column_len();
			let mut new = Array2D::filled_with(Item::Empty, x.column_len(), x.row_len());
			for i in 0..n {
				for j in 0..n {
					new[(i, j)] = x[(n - j - 1, i)];
				}
			}
			new
		}
		Transformation::R2 => transform(&transform(x, Transformation::R1), Transformation::R1),
		Transformation::R3 => transform(&transform(x, Transformation::R1), Transformation::R2),
		Transformation::F1 => transform(&transform(x, Transformation::F4), Transformation::R1),
		Transformation::F2 => transform(&transform(x, Transformation::F3), Transformation::R1),
		Transformation::F3 => {
			let n = x.column_len();
			let mut new = Array2D::filled_with(Item::Empty, x.column_len(), x.row_len());
			for i in 0..n {
				for j in 0..n {
					new[(i, j)] = x[(j, i)];
				}
			}
			new
		}
		Transformation::F4 => transform(&transform(x, Transformation::F3), Transformation::R2),
	}
}

fn parse_tile(s: &str) -> Array2D<Item> {
	Array2D::from_rows(
		&s.lines()
			.map(|x| {
				x.chars()
					.map(|x| match x {
						'.' => Item::Empty,
						'#' => Item::Filled,
						_ => unreachable!(),
					})
					.collect()
			})
			.collect_vec(),
	)
}

fn get_input() -> HashMap<i64, Array2D<Item>> {
	let input = include_str!("20.txt");
	let tiles = input.split("\n\n");
	tiles
		.map(|x| {
			let (header, body) = x.split_once("\n").unwrap();
			let num: i64 = header[5..(header.len() - 1)].parse().unwrap();
			let grid = parse_tile(body);
			(num, grid)
		})
		.collect()
}

fn get_input_transformed() -> HashMap<Id, Array2D<Item>> {
	let all = get_input();
	all.into_iter()
		.flat_map(|(id, grid)| {
			let transforms = all_transforms();
			transforms.into_iter().map(move |x| {
				let new = transform(&grid, x);
				(Id(x, id), new)
			})
		})
		.collect()
}

fn get_borders(x: &Array2D<Item>) -> [Vec<Item>; 4] {
	[
		x.as_rows()[0].clone(),
		x.as_columns().last().unwrap().clone(),
		x.as_rows().last().unwrap().clone(),
		x.as_columns()[0].clone(),
	]
}

fn opposite(border: usize) -> usize {
	assert!(border < 4);
	(border + 2) % 4
}

type BorderMap = MultiMap<Vec<Item>, (Id, Array2D<Item>)>;

fn build_border_map(all: &HashMap<Id, Array2D<Item>>, border: usize) -> BorderMap {
	all.iter()
		.map(|(&num, grid)| {
			let border = get_borders(grid)[border].clone();
			(border, (num, grid.clone()))
		})
		.collect()
}

fn find_aligned(
	all: &HashMap<Id, Array2D<Item>>,
	border_maps: &[BorderMap; 4],
	num: Id,
	border: usize,
) -> Vec<Id> {
	let cur = &all[&num];
	let borders = get_borders(cur);
	let cur_border = borders.get(border);

	if let Some(cur_border) = cur_border {
		let o = opposite(border);
		let want = border_maps[o].get_vec(cur_border);

		want.map(|x| x.iter().map(|x| x.0).collect())
			.unwrap_or_default()
	} else {
		unreachable!()
	}
}

fn all_aligned_nums(
	all: &HashMap<Id, Array2D<Item>>,
	border_maps: &[BorderMap; 4],
	num: Id,
) -> HashSet<i64> {
	let mut result = HashSet::new();
	for i in 0..4 {
		for aligned in find_aligned(&all, &border_maps, num, i) {
			result.insert(aligned.1);
		}
	}
	result.remove(&num.1);
	result
}

fn find_aligned_one(
	all: &HashMap<Id, Array2D<Item>>,
	border_maps: &[BorderMap; 4],
	num: Id,
	border: usize,
) -> Option<Id> {
	let x = find_aligned(&all, &border_maps, num, border)
		.into_iter()
		.filter(|x| x.1 != num.1)
		.collect_vec();
	assert!(x.len() <= 1, "{:?}", x);
	x.get(0).copied()
}

fn is_sea_monster(x: usize, y: usize, grid: &Array2D<Item>) -> bool {
	let schematic = parse_tile(include_str!("sea_monster.txt"));
	if schematic.row_len() + x >= grid.row_len() || schematic.column_len() + y >= grid.column_len()
	{
		return false;
	}
	for i in 0..schematic.row_len() {
		for j in 0..schematic.column_len() {
			if schematic[(j, i)] == Item::Filled && grid[(i + x, j + y)] != Item::Filled {
				return false;
			}
		}
	}
	true
}

fn count_sea_monsters(grid: &Array2D<Item>) -> usize {
	let mut count = 0;
	for i in 0..grid.row_len() {
		for j in 0..grid.column_len() {
			if is_sea_monster(i, j, grid) {
				count += 1;
			}
		}
	}
	count
}

#[util::bench]
fn main() -> (i64, usize) {
	let input = get_input();
	let all = get_input_transformed();

	let border_maps = [
		build_border_map(&all, 0),
		build_border_map(&all, 1),
		build_border_map(&all, 2),
		build_border_map(&all, 3),
	];

	let mut p1 = 1;
	let mut corner_tile = 0;
	for &num in input.keys() {
		let v = all_aligned_nums(&all, &border_maps, Id(Transformation::None, num));
		if v.len() == 2 {
			p1 *= num;
			corner_tile = num;
		}
	}

	let final_image_size = input.len();
	let mut image_map = DefaultHashMap::new(None);
	image_map[Coordinate(0, 0)] = Some(Id(Transformation::None, corner_tile));
	while image_map.len() != final_image_size {
		for (&loc, id) in image_map
			.clone()
			.iter()
			.filter_map(|x| x.1.map(|a| (x.0, a)))
		{
			for &(border, offset) in &[
				(0, Coordinate(0, 1)),
				(1, Coordinate(1, 0)),
				(2, Coordinate(0, -1)),
				(3, Coordinate(-1, 0)),
			] {
				let x = find_aligned_one(&all, &border_maps, id, opposite(border));

				if let Some(up) = x {
					image_map[loc + offset] = Some(up);
				}
			}
		}
	}

	let mut final_image = DefaultHashMap::new(None);
	let tile_width = 8;

	for (&loc, &id) in image_map.iter() {
		let id = id.unwrap();
		let tile = all[&id].clone();
		let tile = transform(&tile, Transformation::R3);
		let dest_coords = loc * tile_width;
		for i in 0..tile_width {
			for j in 0..tile_width {
				final_image[dest_coords + Coordinate(i, j)] =
					Some(tile[((i + 1) as _, (j + 1) as _)]);
			}
		}
	}

	let min_x = final_image.keys().map(|x| x.0).min().unwrap();
	let min_y = final_image.keys().map(|x| x.1).min().unwrap();
	let min = Coordinate(min_x, min_y);

	let size = final_image.keys().map(|x| x.0).max().unwrap()
		- (final_image.keys().map(|x| x.0).min().unwrap())
		+ 1;

	let mut final_image_array = Array2D::filled_with(Item::Empty, size as _, size as _);

	for i in 0..size {
		for j in 0..size {
			let loc = Coordinate(i, j);
			let loc2 = loc + min;
			final_image_array[(loc.0 as usize, loc.1 as usize)] = final_image[loc2].unwrap();
		}
	}

	let mut p2 = 0;

	for t in all_transforms() {
		let transformed_image = transform(&final_image_array, t);
		let count = count_sea_monsters(&transformed_image);
		if count > 0 {
			let total_filled = transformed_image
				.as_row_major()
				.into_iter()
				.filter(|&x| x == Item::Filled)
				.count();
			let sea_monster_size = 15;
			let sea_monster_tiles = sea_monster_size * count;
			p2 = total_filled - sea_monster_tiles;
		}
	}

	(p1, p2)
}
