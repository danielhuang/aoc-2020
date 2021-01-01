#![feature(array_windows)]

use std::{
	collections::HashSet,
	iter::{from_fn, once},
};

struct List {
	inner: Vec<Option<(usize, usize)>>,
}

impl List {
	fn new(data: &[usize]) -> Self {
		let mut inner = vec![None; data.len() + 1];
		for &[a, cur, b] in data.array_windows() {
			inner[cur] = Some((a, b));
		}
		inner[data[0]] = Some((data.last().copied().unwrap(), data[1]));
		let len = data.len();
		inner[data[len - 1]] = Some((data[len - 2], data[0]));
		Self { inner }
	}

	fn remove(&mut self, num: usize) {
		let (before, after) = self.inner[num].unwrap();
		self.inner.get_mut(before).unwrap().as_mut().unwrap().1 = after;
		self.inner.get_mut(after).unwrap().as_mut().unwrap().0 = before;
		self.inner[num] = None;
	}

	fn insert_after(&mut self, target: usize, num: usize) {
		let (_, after) = self.inner[target].unwrap();
		self.inner.get_mut(target).unwrap().as_mut().unwrap().1 = num;
		self.inner.get_mut(after).unwrap().as_mut().unwrap().0 = num;
		self.inner[num] = Some((target, after));
	}

	fn get_after(&self, num: usize) -> usize {
		self.inner[num].unwrap().1
	}

	fn iter(&self, start: usize) -> impl Iterator<Item = usize> {
		let mut cur = start;
		let s = self.inner.clone();
		let iter = from_fn(move || {
			cur = s[cur].unwrap().1;
			if cur == start {
				None
			} else {
				Some(cur)
			}
		});
		once(start).chain(iter)
	}
}

fn get_input() -> Vec<usize> {
	include_str!("23.txt")
		.chars()
		.map(|x| x.to_string().parse().unwrap())
		.collect()
}

fn run(input: &[usize], rounds: usize) -> List {
	let mut list = List::new(input);

	let size = input.len();

	let mut cur = input[0];

	for _ in 0..rounds {
		let a = list.get_after(cur);
		let b = list.get_after(a);
		let c = list.get_after(b);

		list.remove(a);
		list.remove(b);
		list.remove(c);

		let mut dest = (cur - 1).rem_euclid(size);

		if dest == 0 {
			dest = size;
		}

		while [cur, a, b, c].contains(&dest) {
			dest -= 1;
			dest = dest.rem_euclid(size);
			if dest == 0 {
				dest = size;
			}
		}

		list.insert_after(dest, c);
		list.insert_after(dest, b);
		list.insert_after(dest, a);

		cur = list.get_after(cur);
	}

	list
}

#[util::bench]
fn main() -> (String, usize) {
	let mut input = get_input();

	let p1 = run(&input, 100)
		.iter(1)
		.skip(1)
		.map(|x| x.to_string())
		.collect::<String>();

	let orig: HashSet<_> = input.iter().copied().collect();

	for i in 1..=1000000 {
		if !orig.contains(&i) {
			input.push(i);
		}
	}

	let p2 = run(&input, 10000000)
		.iter(1)
		.skip(1)
		.take(2)
		.product::<usize>();

	(p1, p2)
}
