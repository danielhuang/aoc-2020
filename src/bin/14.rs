use std::collections::HashMap;

use fasthash::{sea::Hash64, RandomState};
use itertools::Itertools;

fn get_input() -> &'static str {
	include_str!("14.txt")
}

#[util::bench]
fn main() -> i64 {
	let input = get_input();

	let mut cur_mask = "".to_string();

	let mut mem = HashMap::with_hasher(RandomState::<Hash64>::default());

	for line in input.lines() {
		let (ins, _, val) = line.split(' ').collect_tuple().unwrap();

		match ins {
			"mask" => {
				cur_mask = val.to_string();
			}
			_ => {
				let addr = &ins[4..(ins.len() - 1)];
				let addr: i64 = addr.parse().unwrap();

				let mut addr_str: Vec<_> = format!("{:036b}", addr).chars().collect();

				let val: i64 = val.parse().unwrap();

				let result: Vec<_> = format!("{:036b}", val).chars().collect();

				for (i, c) in cur_mask.chars().enumerate() {
					match c {
						'0' => {}
						'1' => {
							addr_str[i] = '1';
						}
						'X' => {
							addr_str[i] = 'F';
						}
						_ => unimplemented!(),
					}
				}

				let floats = addr_str.iter().filter(|&&x| x == 'F').count() as u32;
				let floating_combos = 2i64.pow(floats);

				for i in 0..floating_combos {
					let addr_edit = format!("{:b}", i);
					let addr_edit = format!("{:0>width$}", addr_edit, width = floats as usize);

					let mut virtual_addr = addr_str.clone();
					for edit in addr_edit.chars() {
						let first_floating = virtual_addr.iter().position(|&x| x == 'F').unwrap();
						virtual_addr[first_floating] = edit;
					}

					mem.insert(
						i64::from_str_radix(&virtual_addr.iter().collect::<String>(), 2).unwrap(),
						result.clone(),
					);
				}
			}
		}
	}

	mem.values()
		.map(|x| i64::from_str_radix(&x.iter().collect::<String>(), 2).unwrap())
		.sum::<i64>()
}
