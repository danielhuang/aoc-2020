#![feature(str_split_once)]

use std::collections::{HashSet, VecDeque};

fn parse(s: &str) -> VecDeque<usize> {
	s.lines().skip(1).map(|x| x.parse().unwrap()).collect()
}

fn get_input() -> (VecDeque<usize>, VecDeque<usize>) {
	let input = include_str!("22.txt");
	let (a, b) = input.split_once("\n\n").unwrap();
	(parse(a), parse(b))
}

fn player_a_wins(
	deck_a: &mut VecDeque<usize>,
	deck_b: &mut VecDeque<usize>,
	recurse: bool,
) -> bool {
	let mut previous = HashSet::new();

	while !deck_a.is_empty() && !deck_b.is_empty() {
		let card_a = deck_a.pop_front().unwrap();
		let card_b = deck_b.pop_front().unwrap();

		if !previous.insert((deck_a.clone(), deck_b.clone())) {
			return true;
		}

		let a_won = if recurse && card_a <= deck_a.len() && card_b <= deck_b.len() {
			let mut deck_a_copy = deck_a.iter().copied().take(card_a).collect();
			let mut deck_b_copy = deck_b.iter().copied().take(card_b).collect();

			player_a_wins(&mut deck_a_copy, &mut deck_b_copy, recurse)
		} else {
			match card_a.cmp(&card_b) {
				std::cmp::Ordering::Less => false,
				std::cmp::Ordering::Greater => true,
				std::cmp::Ordering::Equal => unreachable!(),
			}
		};

		if a_won {
			deck_a.push_back(card_a);
			deck_a.push_back(card_b);
		} else {
			deck_b.push_back(card_b);
			deck_b.push_back(card_a);
		}
	}

	deck_b.is_empty()
}

fn play(recurse: bool) -> usize {
	let (mut deck_a, mut deck_b) = get_input();

	let a_won = player_a_wins(&mut deck_a, &mut deck_b, recurse);

	let winner = if a_won { deck_a } else { deck_b };

	winner
		.iter()
		.rev()
		.enumerate()
		.map(|(n, &x)| (n + 1) * x)
		.sum()
}

#[util::bench]
fn main() -> (usize, usize) {
	(play(false), play(true))
}
