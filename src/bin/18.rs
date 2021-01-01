#![feature(hash_drain_filter)]

fn get_input() -> &'static str {
	include_str!("18.txt")
}

pub fn tokenize(input_str: &str) -> Vec<InfixToken> {
	let mut tokens: Vec<InfixToken> = Vec::new();
	for ele in input_str.split_whitespace() {
		tokens.push(match ele {
			"+" => InfixToken::Op(Operator::Add),
			"-" => InfixToken::Op(Operator::Sub),
			"*" => InfixToken::Op(Operator::Mul),
			"/" => InfixToken::Op(Operator::Div),
			"(" => InfixToken::LeftParen,
			")" => InfixToken::RightParen,
			_ => InfixToken::Num(ele.parse::<i64>().unwrap()),
		});
	}
	tokens
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
	Op(Operator),
	Num(i64),
	LeftParen,
	RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
	Op(Operator),
	Num(i64),
}

pub fn infix_to_postfix(tokens: &[InfixToken]) -> Vec<PostfixToken> {
	let mut stack = Vec::new();
	let mut result: Vec<PostfixToken> = Vec::new();
	for token in tokens {
		if let InfixToken::Num(value) = token {
			result.push(PostfixToken::Num(*value));
		} else if let InfixToken::LeftParen = token {
			stack.push(token);
		} else if let InfixToken::RightParen = token {
			loop {
				let popout_token = stack.pop().unwrap();
				if let InfixToken::LeftParen = popout_token {
					break;
				} else if let InfixToken::Op(operator) = popout_token {
					result.push(PostfixToken::Op(*operator));
				}
			}
		} else if let InfixToken::Op(operator) = token {
			if stack.is_empty() {
				stack.push(token);
			} else {
				loop {
					if !stack.is_empty() {
						let top_token = stack[stack.len() - 1];

						if let InfixToken::Op(stack_operator) = top_token {
							if *stack_operator == Operator::Add || *stack_operator == Operator::Sub
							{
								if *operator == Operator::Sub || *operator == Operator::Add {
									if let InfixToken::Op(operator4out) = stack.pop().unwrap() {
										result.push(PostfixToken::Op(*operator4out));
									} else {
										break;
									}
								} else {
									break;
								}
							} else if *stack_operator == Operator::Mul
								|| *stack_operator == Operator::Div
							{
								if let InfixToken::Op(operator4out) = stack.pop().unwrap() {
									result.push(PostfixToken::Op(*operator4out));
								} else {
									break;
								}
							}
						} else {
							break;
						}
					} else {
						break;
					}
				}
				stack.push(token);
			}
		}
	}

	for _ in 0..stack.len() {
		if let InfixToken::Op(operator) = stack.pop().unwrap() {
			result.push(PostfixToken::Op(*operator));
		}
	}
	result
}

fn infix_to_postfix2(s: &str) -> Vec<PostfixToken> {
	let input_str = s.to_string();
	let original_tokens = tokenize(&input_str);

	infix_to_postfix(&original_tokens)
}

fn run(input: &str, p2: bool) -> i64 {
	let mut sum = 0;

	for line in input.lines() {
		let mut line = line
			.replace("*", "-")
			.replace("(", " ( ")
			.replace(")", " ) ");

		if p2 {
			line = line.replace("+", "/");
		}

		let postfix = infix_to_postfix2(&line);

		let mut stack = vec![];

		for token in postfix {
			match token {
				PostfixToken::Op(op) => {
					let a = stack.pop().unwrap();
					let b = stack.pop().unwrap();
					match op {
						Operator::Add => {
							stack.push(a + b);
						}
						Operator::Sub => {
							stack.push(a * b);
						}
						Operator::Mul => unimplemented!(),
						Operator::Div => {
							stack.push(a + b);
						}
					}
				}
				PostfixToken::Num(x) => stack.push(x),
			}
		}

		sum += stack[0];
	}

	sum
}

#[util::bench]
fn main() -> (i64, i64) {
	let input = get_input();

	(run(input, false), run(input, true))
}
