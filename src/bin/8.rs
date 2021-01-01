use std::collections::HashSet;

use itertools::Itertools;

fn parse(s: &str) -> Instruction {
	let (ins, val) = s.split(' ').collect_tuple().unwrap();
	let val = val.parse().unwrap();
	match ins {
		"acc" => Instruction::Acc(val),
		"jmp" => Instruction::Jmp(val),
		"nop" => Instruction::Nop(val),
		_ => unimplemented!(),
	}
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
	Acc(i32),
	Jmp(i32),
	Nop(i32),
}

fn run(texts: &[Instruction]) -> Option<i32> {
	let mut pos = 0;
	let mut acc = 0;
	let mut ran = HashSet::new();
	loop {
		if pos == texts.len() {
			break;
		}

		let ins = texts[pos];

		if ran.contains(&pos) {
			return None;
		}

		ran.insert(pos);

		match ins {
			Instruction::Acc(x) => {
				acc += x;
				pos += 1;
			}
			Instruction::Jmp(x) => {
				pos = (pos as i32 + x) as usize;
			}
			Instruction::Nop(_) => {
				pos += 1;
			}
		}
	}
	Some(acc)
}

#[util::bench]
fn main() -> i32 {
	let input = include_str!("8.txt");
	let texts: Vec<_> = input.split('\n').map(|x| parse(x)).collect();

	for i in 0..(texts.len()) {
		let mut cloned = texts.clone();
		match cloned[i] {
			Instruction::Acc(_) => {}
			Instruction::Jmp(x) => cloned[i] = Instruction::Nop(x),
			Instruction::Nop(x) => cloned[i] = Instruction::Jmp(x),
		}
		let ee = run(&cloned);
		if let Some(x) = ee {
			return x;
		}
	}

	unreachable!()
}
