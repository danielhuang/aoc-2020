use itertools::Itertools;

fn get_input() -> &'static str {
	include_str!("13.txt")
}

fn egcd(n1: i64, n2: i64) -> (i64, i64, i64) {
	if n1 == 0 {
		(n2, 0, 1)
	} else {
		let (g, x, y) = egcd(n2 % n1, n1);
		(g, y - (n2 / n1) * x, x)
	}
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
	let (g, x, _) = egcd(x, n);
	if g == 1 {
		Some((x % n + n) % n)
	} else {
		None
	}
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
	let prod = modulii.iter().product::<i64>();

	let mut sum = 0;

	for (&residue, &modulus) in residues.iter().zip(modulii) {
		let p = prod / modulus;
		sum += residue * mod_inv(p, modulus)? * p
	}

	Some(sum % prod)
}

#[util::bench]
fn main() -> (i64, i64) {
	let input = get_input();

	let (start_time, buses) = input.lines().collect_tuple().unwrap();
	let start_time: i64 = start_time.parse().unwrap();

	let available_buses: Vec<i64> = buses.split(',').filter_map(|x| x.parse().ok()).collect();

	let bus = available_buses
		.iter()
		.copied()
		.min_by_key(|x| x - (start_time % x))
		.unwrap();

	let time = bus - (start_time % bus);

	let timetable: Vec<(usize, Option<i64>)> = buses
		.split(',')
		.enumerate()
		.map(|(i, x)| (i, x.parse().ok()))
		.collect();

	let r = chinese_remainder(
		&timetable
			.iter()
			.copied()
			.filter_map(|(i, x)| x.map(|x| if i == 0 { 0 } else { x } - i as i64))
			.collect_vec(),
		&timetable
			.iter()
			.copied()
			.filter_map(|(_i, x)| x)
			.collect_vec(),
	)
	.unwrap();

	(bus * time, r)
}
