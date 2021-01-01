fn get_input() -> Vec<usize> {
	include_str!("15.txt")
		.split(',')
		.map(|x| x.parse().unwrap())
		.collect()
}

fn run(num: usize) -> usize {
	let mut input = get_input();

	let mut cache = vec![None; num];

	for (i, &x) in input.iter().enumerate() {
		cache[x] = Some(i);
	}

	while input.len() < num {
		let last_num = input.iter().copied().last().unwrap();
		let pos = cache[last_num];
		if let Some(pos) = pos {
			let dif = input.len() - pos - 1;
			input.push(dif);
		} else {
			input.push(0);
		}
		cache[input.iter().copied().rev().nth(1).unwrap()] = Some(input.len() - 2);
	}

	input[num - 1]
}

#[util::bench]
fn main() -> (usize, usize) {
	(run(2020), run(30000000))
}
