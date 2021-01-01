use aoc::Coordinate;

fn get_input() -> Vec<&'static str> {
	let input = include_str!("12.txt");
	input.lines().collect()
}

fn rotate_once_right(coordinate: Coordinate) -> Coordinate {
	Coordinate(coordinate.1, -coordinate.0)
}

#[util::bench]
fn main() -> i64 {
	let input = get_input();

	let mut pos = Coordinate(0, 0);

	let mut waypoint = Coordinate(10, 1);

	for &line in &input {
		let num: i64 = line.chars().skip(1).collect::<String>().parse().unwrap();
		let ins = line.chars().next().unwrap();
		match ins {
			'E' => {
				waypoint += Coordinate(num, 0);
			}
			'W' => {
				waypoint += Coordinate(-num, 0);
			}
			'N' => {
				waypoint += Coordinate(0, num);
			}
			'S' => {
				waypoint += Coordinate(0, -num);
			}
			c @ 'R' | c @ 'L' => {
				let degrees: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();
				let turns = degrees / 90;
				let turns_right = match c {
					'R' => 1,
					'L' => 3,
					_ => unimplemented!(),
				} * turns;
				for _ in 0..turns_right {
					waypoint = rotate_once_right(waypoint);
				}
			}
			'F' => {
				pos += waypoint * num;
			}
			_ => unimplemented!(),
		};
	}

	pos.0.abs() + pos.1.abs()
}
