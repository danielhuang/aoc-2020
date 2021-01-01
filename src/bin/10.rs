use defaultmap::DefaultHashMap;
use itertools::Itertools;

fn get_input() -> Vec<i64> {
	let input = include_str!("10.txt");
	let nums: Vec<_> = input
		.split('\n')
		.map(|x| x.parse::<i64>().unwrap())
		.collect();
	nums
}

fn find_differences(nums: &[i64]) -> Vec<i64> {
	let differences = nums
		.iter()
		.copied()
		.tuple_windows()
		.map(|(a, b)| b - a)
		.collect_vec();
	differences
}

fn count_num(s: i64) -> usize {
	(0..2_i64.pow((s - 1) as u32))
		.map(|x| format!("{:b}", x))
		.filter(|x| !x.contains("111"))
		.count()
}

#[util::bench]
fn main() -> usize {
	let mut nums = get_input();

	nums.insert(0, 0);

	nums.sort_unstable();

	let differences = find_differences(&nums);

	let mut map = DefaultHashMap::new(0);

	for &dif in &differences {
		map[dif] += 1;
	}

	map[1] += 1;
	map[3] += 1;

	let mut total = 1;

	for (count, &num) in differences.iter().dedup_with_count() {
		if num == 1 {
			total *= count_num(count as i64);
		}
	}

	total
}
