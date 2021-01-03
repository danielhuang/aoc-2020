fn get_input() -> Vec<i32> {
	include_str!("15.txt")
		.split(',')
		.map(|x| x.parse().unwrap())
		.collect()
}

fn run(num: i32) -> i32 {
	let mut input = get_input();

	let mut cache = vec![-1; num as usize];

	for (i, &x) in input.iter().enumerate() {
		cache[x as usize] = i as i32;
	}

	while input.len() < num as usize {
		let last_num = input.iter().copied().last().unwrap();
		let pos = cache[last_num as usize];
		if pos != -1 {
			let dif = input.len() as i32 - pos - 1;
			input.push(dif);
		} else {
			input.push(0);
		}
		cache[input.iter().copied().rev().nth(1).unwrap() as usize] = input.len() as i32 - 2;
	}

	input[num as usize - 1]
}

#[util::bench]
fn main() -> (i32, i32) {
	(run(2020), run(30000000))
}
