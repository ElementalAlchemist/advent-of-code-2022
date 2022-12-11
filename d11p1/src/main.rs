use std::collections::BTreeMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy)]
enum OperationType {
	Add,
	Multiply,
}

impl OperationType {
	fn apply(&self, operand1: u32, operand2: u32) -> u32 {
		match self {
			Self::Add => operand1 + operand2,
			Self::Multiply => operand1 * operand2,
		}
	}
}

#[derive(Clone, Copy)]
enum OperationBy {
	OldValue,
	Number(u32),
}

#[derive(Clone, Copy)]
struct Operation {
	op: OperationType,
	by: OperationBy,
}

impl Operation {
	fn apply(&self, operand: u32) -> u32 {
		let by = match self.by {
			OperationBy::OldValue => operand,
			OperationBy::Number(num) => num,
		};
		self.op.apply(by, operand)
	}
}

struct Monkey {
	items: Vec<u32>,
	worry_operation: Operation,
	test_modulus: u32,
	true_destination: usize,
	false_destination: usize,
	throws_made: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut monkeys = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut input_lines = input_string.lines().filter(|s| !s.is_empty());
		// Assume the monkeys are in order
		let mut monkey_items: Vec<u32> = Vec::new();
		let mut monkey_operation = Operation {
			op: OperationType::Add,
			by: OperationBy::Number(0),
		};
		let mut test_modulus: u32 = 1;
		let mut true_destination: usize = 0;
		let mut false_destination: usize = 0;

		input_lines.next(); // Eat the first monkey's header

		let mut monkeys: Vec<Monkey> = Vec::new();

		for line in input_lines {
			if line.starts_with("Monkey ") {
				let new_monkey = Monkey {
					items: monkey_items.clone(),
					worry_operation: monkey_operation,
					test_modulus,
					true_destination,
					false_destination,
					throws_made: 0,
				};
				monkeys.push(new_monkey);
				monkey_items.clear();
			} else if let Some(items) = line.strip_prefix("  Starting items: ") {
				monkey_items = items.split(", ").map(|item| item.parse().unwrap()).collect();
			} else if let Some(operation) = line.strip_prefix("  Operation: new = old ") {
				let mut parts = operation.split(' ');
				let operator = parts.next().unwrap();
				let amount = parts.next().unwrap();
				assert!(parts.next().is_none(), "All data has been parsed");
				let operator = match operator {
					"+" => OperationType::Add,
					"*" => OperationType::Multiply,
					_ => panic!("Unhandled operation type: {}", operator),
				};
				let amount = match amount {
					"old" => OperationBy::OldValue,
					_ => OperationBy::Number(amount.parse()?),
				};
				monkey_operation = Operation {
					op: operator,
					by: amount,
				};
			} else if let Some(modulus) = line.strip_prefix("  Test: divisible by ") {
				test_modulus = modulus.parse().unwrap();
			} else if let Some(dest) = line.strip_prefix("    If true: throw to monkey ") {
				true_destination = dest.parse().unwrap();
			} else if let Some(dest) = line.strip_prefix("    If false: throw to monkey ") {
				false_destination = dest.parse().unwrap();
			} else {
				eprintln!("Unparsed line: [{}]", line);
			}
		}
		monkeys.push(Monkey {
			items: monkey_items,
			worry_operation: monkey_operation,
			test_modulus,
			true_destination,
			false_destination,
			throws_made: 0,
		});

		monkeys
	};

	let mut append_to_true: Vec<u32> = Vec::new();
	let mut append_to_false: Vec<u32> = Vec::new();

	for _ in 0..20 {
		for monkey_index in 0..monkeys.len() {
			for item in monkeys[monkey_index].items.iter() {
				let new_worry = monkeys[monkey_index].worry_operation.apply(*item) / 3;
				if new_worry % monkeys[monkey_index].test_modulus == 0 {
					append_to_true.push(new_worry);
				} else {
					append_to_false.push(new_worry);
				};
			}
			monkeys[monkey_index].throws_made += monkeys[monkey_index].items.len();
			monkeys[monkey_index].items.clear();

			let true_destination = monkeys[monkey_index].true_destination;
			let false_destination = monkeys[monkey_index].false_destination;

			for item in append_to_true.drain(..) {
				monkeys[true_destination].items.push(item);
			}
			for item in append_to_false.drain(..) {
				monkeys[false_destination].items.push(item);
			}
		}
	}

	let monkey_throws: BTreeMap<usize, usize> = monkeys
		.iter()
		.enumerate()
		.map(|(index, monkey)| (monkey.throws_made, index))
		.collect();
	let mut monkey_throw_iter = monkey_throws.iter().rev();
	let most_throws = monkey_throw_iter.next().unwrap().0;
	let second_most_throws = monkey_throw_iter.next().unwrap().0;

	let monkey_business = most_throws * second_most_throws;
	println!("{}", monkey_business);

	Ok(())
}
