#![feature(str_split_once)]

#[util::bench]
fn main() -> i64 {
	let (card, door) = include_str!("25.txt").split_once('\n').unwrap();

	let card_public_key: i64 = card.parse().unwrap();
	let door_public_key: i64 = door.parse().unwrap();

	let mut n = 1;
	let mut card_loop_size = 0;
	while n != card_public_key {
		n = n * 7 % 20201227;
		card_loop_size += 1;
	}

	let mut n2 = 1;
	for _ in 0..card_loop_size {
		n2 = n2 * door_public_key % 20201227;
	}

	n2
}
