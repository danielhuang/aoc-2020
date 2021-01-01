#![feature(str_split_once)]

use std::collections::{HashMap, HashSet};

fn get_input() -> Vec<(HashSet<&'static str>, Vec<&'static str>)> {
	include_str!("21.txt")
		.lines()
		.map(|line| {
			let (ingredients, allergens) =
				line[0..(line.len() - 1)].split_once(" (contains ").unwrap();
			let ingredients: HashSet<_> = ingredients.split(' ').collect();
			let allergens: Vec<_> = allergens.split(", ").collect();
			(ingredients, allergens)
		})
		.collect()
}

#[util::bench]
fn main() -> (usize, String) {
	let input = get_input();

	let mut possibilities: HashMap<&str, HashSet<&str>> = HashMap::new();
	for (ingredients, allergens) in input.clone() {
		for &allergen in &allergens {
			if let Some(p) = possibilities.get_mut(allergen) {
				*p = p.intersection(&ingredients).copied().collect();
			} else {
				possibilities.insert(allergen, ingredients.clone());
			}
		}
	}

	while !possibilities.values().all(|x| x.len() <= 1) {
		for (&allergen, ingredients) in possibilities.clone().iter() {
			if ingredients.len() == 1 {
				let &ingredient = ingredients.iter().next().unwrap();
				for &allergen2 in possibilities.clone().keys() {
					if allergen != allergen2 {
						possibilities.get_mut(allergen2).unwrap().remove(ingredient);
					}
				}
			}
		}
	}

	let all_allergic: HashSet<_> = possibilities.values().flatten().copied().collect();
	let all_ingredients = input.iter().flat_map(|x| x.0.clone());

	let p1 = all_ingredients
		.filter(|x| !all_allergic.contains(x))
		.count();

	let mut all_allergic: Vec<_> = possibilities.into_iter().collect();
	all_allergic.sort_unstable_by_key(|x| x.0);

	let p2 = all_allergic
		.into_iter()
		.map(|x| x.1.into_iter().next().unwrap())
		.collect::<Vec<_>>()
		.join(",");

	(p1, p2)
}
