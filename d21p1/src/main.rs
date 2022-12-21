use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;

enum Monkey {
	Number(u64),
	Add(Box<Monkey>, Box<Monkey>),
	Subtract(Box<Monkey>, Box<Monkey>),
	Multiply(Box<Monkey>, Box<Monkey>),
	Divide(Box<Monkey>, Box<Monkey>),
}

impl Monkey {
	fn number(&self) -> u64 {
		match self {
			Self::Number(num) => *num,
			Self::Add(lhm, rhm) => {
				let lhs = lhm.number();
				let rhs = rhm.number();
				lhs + rhs
			}
			Self::Subtract(lhm, rhm) => {
				let lhs = lhm.number();
				let rhs = rhm.number();
				lhs - rhs
			}
			Self::Multiply(lhm, rhm) => {
				let lhs = lhm.number();
				let rhs = rhm.number();
				lhs * rhs
			}
			Self::Divide(lhm, rhm) => {
				let lhs = lhm.number();
				let rhs = rhm.number();
				lhs / rhs
			}
		}
	}
}

enum MonkeySpec {
	Number(u64),
	Add(String, String),
	Subtract(String, String),
	Multiply(String, String),
	Divide(String, String),
}

fn build_monkey_from_spec(specs: &HashMap<String, MonkeySpec>, root: &str) -> Monkey {
	match specs.get(root).unwrap() {
		MonkeySpec::Number(num) => Monkey::Number(*num),
		MonkeySpec::Add(lhm, rhm) => {
			let lhs = build_monkey_from_spec(specs, lhm);
			let rhs = build_monkey_from_spec(specs, rhm);
			Monkey::Add(Box::new(lhs), Box::new(rhs))
		}
		MonkeySpec::Subtract(lhm, rhm) => {
			let lhs = build_monkey_from_spec(specs, lhm);
			let rhs = build_monkey_from_spec(specs, rhm);
			Monkey::Subtract(Box::new(lhs), Box::new(rhs))
		}
		MonkeySpec::Multiply(lhm, rhm) => {
			let lhs = build_monkey_from_spec(specs, lhm);
			let rhs = build_monkey_from_spec(specs, rhm);
			Monkey::Multiply(Box::new(lhs), Box::new(rhs))
		}
		MonkeySpec::Divide(lhm, rhm) => {
			let lhs = build_monkey_from_spec(specs, lhm);
			let rhs = build_monkey_from_spec(specs, rhm);
			Monkey::Divide(Box::new(lhs), Box::new(rhs))
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let monkey_specs: HashMap<String, MonkeySpec> = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut monkey_specs = HashMap::new();

		for line in input_string.lines().filter(|s| !s.is_empty()) {
			let mut parts = line.split(": ");
			let monkey_name = String::from(parts.next().unwrap());
			let value = parts.next().unwrap();
			assert!(parts.next().is_none());

			let parse_result: Result<u64, ParseIntError> = value.parse();
			let monkey_value = match parse_result {
				Ok(num) => MonkeySpec::Number(num),
				Err(_) => {
					let mut parts = value.split(' ');
					let lhs = parts.next().unwrap().to_owned();
					let operator = parts.next().unwrap();
					let rhs = parts.next().unwrap().to_owned();
					assert!(parts.next().is_none());
					match operator {
						"+" => MonkeySpec::Add(lhs, rhs),
						"-" => MonkeySpec::Subtract(lhs, rhs),
						"*" => MonkeySpec::Multiply(lhs, rhs),
						"/" => MonkeySpec::Divide(lhs, rhs),
						_ => panic!("Unexpected operator {}", operator),
					}
				}
			};

			monkey_specs.insert(monkey_name, monkey_value);
		}

		monkey_specs
	};

	let monkey = build_monkey_from_spec(&monkey_specs, "root");
	let number = monkey.number();
	println!("{}", number);

	Ok(())
}
