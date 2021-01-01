use itertools::Itertools;

#[util::bench]
fn main() -> i32 {
	let input = include_str!("4.txt");
	let texts: Vec<_> = input.split("\n\n").collect();

	let mut count = 0;

	for text in texts {
		if check(text) {
			count += 1;
		}
	}

	count
}

fn check(text: &str) -> bool {
	let text = text.replace("\n", " ");
	let subtexts = text.split(' ').collect_vec();

	let has = subtexts.iter().map(|&x| x[0..4].to_string()).collect_vec();

	let pre = if has.len() == 8 {
		true
	} else {
		has.len() == 7 && !has.contains(&"cid:".to_string())
	};

	if !pre {
		return false;
	}

	for subtext in subtexts {
		let prefix = subtext[0..3].to_string();
		let suffix = subtext[4..].to_string();
		match prefix.as_str() {
			"byr" => {
				let num: Result<i32, _> = suffix.parse();
				if let Ok(num) = num {
					if !(1920..=2002).contains(&num) {
						return false;
					}
				} else {
					return false;
				}
			}
			"iyr" => {
				let num: Result<i32, _> = suffix.parse();
				if let Ok(num) = num {
					if !(2010..=2020).contains(&num) {
						return false;
					}
				} else {
					return false;
				}
			}
			"eyr" => {
				let num: Result<i32, _> = suffix.parse();
				if let Ok(num) = num {
					if !(2020..=2030).contains(&num) {
						return false;
					}
				} else {
					return false;
				}
			}
			"hgt" => {
				let num: Result<i32, _> = suffix[0..(suffix.len() - 2)].parse();
				if let Ok(num) = num {
					let unit = suffix[(suffix.len() - 2)..].to_string();
					match unit.as_str() {
						"cm" => {
							if !(150..=193).contains(&num) {
								return false;
							}
						}
						"in" => {
							if !(59..=76).contains(&num) {
								return false;
							}
						}
						_ => {
							return false;
						}
					}
				} else {
					return false;
				}
			}
			"hcl" => {
				if !suffix.starts_with('#') {
					return false;
				}
				if suffix.len() != 7 {
					return false;
				}
				if !suffix
					.chars()
					.skip(1)
					.all(|x| ('0'..='9').contains(&x) || ('a'..='f').contains(&x))
				{
					return false;
				}
			}
			"ecl" => {
				let valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
				if !valid.contains(&suffix.as_str()) {
					return false;
				}
			}
			"pid" => {
				if suffix.len() != 9 {
					return false;
				}
				if !suffix.chars().all(|x| x.is_numeric()) {
					return false;
				}
			}
			"cid" => {}
			_ => unimplemented!(),
		}
	}

	true
}
