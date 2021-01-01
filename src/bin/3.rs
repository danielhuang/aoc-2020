#[util::bench]
fn main() -> (usize, usize) {
	let count_1_1 = check(1, 1);
	let count_3_1 = check(3, 1);
	let count_5_1 = check(5, 1);
	let count_7_1 = check(7, 1);
	let count_1_2 = check(1, 2);
	(
		count_3_1,
		count_1_1 * count_3_1 * count_5_1 * count_7_1 * count_1_2,
	)
}

fn check(dx: usize, dy: usize) -> usize {
	let input = include_str!("3.txt");
	let texts: Vec<_> = input.lines().collect();

	let mut count = 0;

	let mut x = 0;

	for text in texts.into_iter().step_by(dy) {
		if get(text, x) == '#' {
			count += 1;
		}
		x += dx;
	}

	count
}

fn get(line: &str, index: usize) -> char {
	line.chars().cycle().nth(index).unwrap()
}
