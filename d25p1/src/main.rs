use std::error::Error;
use std::fs;
use std::iter::zip;

fn add(lhs: &str, rhs: &str) -> String {
	let mut lhs = lhs.to_string();
	let mut rhs = rhs.to_string();

	while lhs.len() < rhs.len() {
		lhs = format!("0{}", lhs);
	}
	while rhs.len() < lhs.len() {
		rhs = format!("0{}", rhs);
	}

	let mut result = String::new();
	for (l, r) in zip(lhs.chars(), rhs.chars()) {}
	result
}

fn main() -> Result<(), Box<dyn Error>> {
	let amounts: Vec<i64> = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut amounts = Vec::new();

		for line in input_string.lines() {
			let mut number = 0;
			for c in line.chars() {
				number *= 5;
				match c {
					'2' => number += 2,
					'1' => number += 1,
					'0' => (),
					'-' => number -= 1,
					'=' => number -= 2,
					_ => panic!("Unexpected char {}", c),
				}
			}
			amounts.push(number);
		}

		amounts
	};

	let mut sum: i64 = amounts.iter().sum();
	println!("{}", sum);

	let mut partial_snafu_sum = String::new();
	let mut carry = 0;
	while sum > 1 {
		let next_digit = sum % 5 + carry;
		carry = 0;
		let next_char = match next_digit {
			0 => '0',
			1 => '1',
			2 => '2',
			3 => {
				carry = 1;
				'='
			}
			4 => {
				carry = 1;
				'-'
			}
			5 => {
				carry = 1;
				'0'
			}
			6 => {
				carry = 1;
				'1'
			}
			7 => {
				carry = 1;
				'2'
			}
			8 => {
				carry = 2;
				'='
			}
			9 => {
				carry = 2;
				'-'
			}
			10 => {
				carry = 2;
				'0'
			}
			_ => panic!("Increase next char handling")
		};
		partial_snafu_sum.push(next_char);
		sum /= 5;
	}
	match carry {
		0 => (),
		1 => partial_snafu_sum.push('2'),
		_ => panic!("Increase carry handling")
	}

	partial_snafu_sum = partial_snafu_sum.chars().rev().collect();

	let mut snafu_sum = partial_snafu_sum;
	/*for c in partial_snafu_sum.chars() {
		match c {
			'0' | '1' | '2' => snafu_sum.push(c),
			'3' | '4' => {
				let next_char = if c == '3' { '=' } else { '-' };
				let mut carry_stack = Vec::new();
				let mut carry = if c == '3' { 1 } else { 2 };
				while let Some(carry_char) = snafu_sum.pop() {
					match carry {
						0 => carry_stack.push(carry_char),
						1 => {
							carry = 0;
							let new_char = match carry_char {
								'2' => {
									carry = 1;
									'='
								}
								'1' => '2',
								'0' => '1',
								'-' => '0',
								'=' => '-',
								_ => unreachable!(),
							};
							carry_stack.push(new_char);
						}
						2 => {
							carry = 0;
							let new_char = match carry_char {
								'2' => {
									carry = 2;
									'='
								}
								'1' => {
									carry = 1;
									'='
								}
								'0' => '2',
								'-' => '1',
								'=' => '0',
								_ => unreachable!(),
							};
							carry_stack.push(new_char);
						}
						_ => unreachable!(),
					}
				}
				if carry == 2 {
					carry_stack.push('2');
				} else if carry == 1 {
					carry_stack.push('1');
				}
				while let Some(carry_char) = carry_stack.pop() {
					snafu_sum.push(carry_char);
				}
				snafu_sum.push(next_char);
			}
			_ => unreachable!(),
		}
	}*/

	println!("{}", snafu_sum);

	Ok(())
}
