use itertools::Itertools;

#[util::bench]
fn main() -> usize {
	let input = include_str!("2.txt");
	let texts: Vec<_> = input.lines().collect();

	let mut count = 0;

	for text in texts {
		let (nums, letter, pw) = text.split(' ').collect_tuple().unwrap();
		let letter = letter.chars().next().unwrap();

		let (num1, num2) = nums
			.split('-')
			.map(|x| x.parse::<usize>().unwrap())
			.collect_tuple()
			.unwrap();

		let pw = pw.chars().collect_vec();

		if (pw[num1 - 1] == letter) ^ (pw[num2 - 1] == letter) {
			count += 1;
		}
	}

	count
}
