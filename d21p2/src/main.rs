use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;

enum Monkey {
	Number(i64),
	Add(Box<Monkey>, Box<Monkey>),
	Subtract(Box<Monkey>, Box<Monkey>),
	Multiply(Box<Monkey>, Box<Monkey>),
	Divide(Box<Monkey>, Box<Monkey>),
	ShouldEqual(Box<Monkey>, Box<Monkey>),
	UnknownInput,
}

impl Monkey {
	fn number(&self) -> i64 {
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
			Self::ShouldEqual(lhm, rhm) => {
				let mut known_side = lhm;
				let mut unknown_side = rhm;
				let known_side_has_unknown = known_side.has_unknown_input();
				let unknown_side_has_unknown = unknown_side.has_unknown_input();
				assert_ne!(known_side_has_unknown, unknown_side_has_unknown);

				if known_side_has_unknown {
					std::mem::swap(&mut known_side, &mut unknown_side);
				}

				let target_number = known_side.number();
				unknown_side.number_target_unknown(target_number)
			}
			Self::UnknownInput => panic!(),
		}
	}

	fn has_unknown_input(&self) -> bool {
		match self {
			Self::Number(_) => false,
			Self::UnknownInput => true,
			Self::Add(lhs, rhs) => lhs.has_unknown_input() || rhs.has_unknown_input(),
			Self::Subtract(lhs, rhs) => lhs.has_unknown_input() || rhs.has_unknown_input(),
			Self::Multiply(lhs, rhs) => lhs.has_unknown_input() || rhs.has_unknown_input(),
			Self::Divide(lhs, rhs) => lhs.has_unknown_input() || rhs.has_unknown_input(),
			Self::ShouldEqual(lhs, rhs) => lhs.has_unknown_input() || rhs.has_unknown_input(),
		}
	}

	fn number_target_unknown(&self, equal_target: i64) -> i64 {
		match self {
			Self::Number(num) => *num,
			Self::Add(lhm, rhm) => {
				let mut known_side = lhm;
				let mut unknown_side = rhm;
				let known_side_has_unknown = known_side.has_unknown_input();
				let unknown_side_has_unknown = unknown_side.has_unknown_input();
				assert_ne!(known_side_has_unknown, unknown_side_has_unknown);
				if known_side_has_unknown {
					std::mem::swap(&mut known_side, &mut unknown_side);
				}
				let target = known_side.number();
				unknown_side.number_target_unknown(equal_target - target)
			}
			Self::Subtract(lhm, rhm) => {
				let mut known_side = lhm;
				let mut unknown_side = rhm;
				let known_side_has_unknown = known_side.has_unknown_input();
				let unknown_side_has_unknown = unknown_side.has_unknown_input();
				assert_ne!(known_side_has_unknown, unknown_side_has_unknown);
				if known_side_has_unknown {
					std::mem::swap(&mut known_side, &mut unknown_side);
				}
				let target = known_side.number();
				let target = if known_side_has_unknown {
					// RHS is known side
					equal_target + target
				} else {
					-(equal_target - target)
				};
				unknown_side.number_target_unknown(target)
			}
			Self::Multiply(lhm, rhm) => {
				let mut known_side = lhm;
				let mut unknown_side = rhm;
				let known_side_has_unknown = known_side.has_unknown_input();
				let unknown_side_has_unknown = unknown_side.has_unknown_input();
				assert_ne!(known_side_has_unknown, unknown_side_has_unknown);
				if known_side_has_unknown {
					std::mem::swap(&mut known_side, &mut unknown_side);
				}
				let target = known_side.number();
				let target = equal_target / target;
				unknown_side.number_target_unknown(target)
			}
			Self::Divide(lhm, rhm) => {
				let mut known_side = lhm;
				let mut unknown_side = rhm;
				let known_side_has_unknown = known_side.has_unknown_input();
				let unknown_side_has_unknown = unknown_side.has_unknown_input();
				assert_ne!(known_side_has_unknown, unknown_side_has_unknown);
				if known_side_has_unknown {
					std::mem::swap(&mut known_side, &mut unknown_side);
				}
				let target = known_side.number();
				let target = if known_side_has_unknown {
					// RHS is known side
					equal_target * target
				} else {
					target / equal_target
				};
				unknown_side.number_target_unknown(target)
			}
			Self::ShouldEqual(_, _) => panic!(),
			Self::UnknownInput => equal_target,
		}
	}
}

enum MonkeySpec {
	Number(i64),
	Add(String, String),
	Subtract(String, String),
	Multiply(String, String),
	Divide(String, String),
	ShouldEqual(String, String),
	UnknownInput,
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
		MonkeySpec::ShouldEqual(lhm, rhm) => {
			let lhs = build_monkey_from_spec(specs, lhm);
			let rhs = build_monkey_from_spec(specs, rhm);
			Monkey::ShouldEqual(Box::new(lhs), Box::new(rhs))
		}
		MonkeySpec::UnknownInput => Monkey::UnknownInput,
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

			let parse_result: Result<i64, ParseIntError> = value.parse();
			let monkey_value = match parse_result {
				Ok(num) => {
					if monkey_name == "humn" {
						MonkeySpec::UnknownInput
					} else {
						MonkeySpec::Number(num)
					}
				}
				Err(_) => {
					let mut parts = value.split(' ');
					let lhs = parts.next().unwrap().to_owned();
					let operator = parts.next().unwrap();
					let rhs = parts.next().unwrap().to_owned();
					assert!(parts.next().is_none());
					if monkey_name == "root" {
						MonkeySpec::ShouldEqual(lhs, rhs)
					} else {
						match operator {
							"+" => MonkeySpec::Add(lhs, rhs),
							"-" => MonkeySpec::Subtract(lhs, rhs),
							"*" => MonkeySpec::Multiply(lhs, rhs),
							"/" => MonkeySpec::Divide(lhs, rhs),
							_ => panic!("Unexpected operator {}", operator),
						}
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
