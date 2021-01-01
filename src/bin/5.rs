use std::collections::HashSet;

#[util::bench]
fn main() -> (i32, i32) {
	let input = include_str!("5.txt");
	let texts: Vec<_> = input.split('\n').collect();

	let max = texts.iter().map(|&x| seat(x)).max().unwrap();

	let seats: HashSet<_> = texts.iter().copied().map(|x| seat(x)).collect();

	let mut p2 = 0;

	for i in 1..=max {
		if !seats.contains(&i) && seats.contains(&(i - 1)) && seats.contains(&(i + 1)) {
			p2 = i
		}
	}

	(max, p2)
}

fn seat(s: &str) -> i32 {
	let processed: String = s
		.chars()
		.map(|x| match x {
			'B' | 'R' => '1',
			'F' | 'L' => '0',
			_ => unimplemented!(),
		})
		.collect();

	i32::from_str_radix(&processed, 2).unwrap()
}
