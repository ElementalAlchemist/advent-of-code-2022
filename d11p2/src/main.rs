use std::collections::BinaryHeap;
use std::error::Error;
use std::fs;

const DEFAULT_WORRY_MOD: u64 = 1007;

#[derive(Clone, Copy)]
struct WorryAmount {
	basis: u64,
	mult: u64,
}

impl WorryAmount {
	fn new(num: u64, worry_mod: u64) -> Self {
		let basis = num % worry_mod;
		let mult = num / worry_mod;
		Self { basis, mult }
	}

	fn is_divisible(&self, modulus: u64, worry_mod: u64) -> bool {
		((worry_mod % modulus) * self.mult + self.basis) % modulus == 0
	}

	fn rebalance(&self, old_worry_mod: u64, new_worry_mod: u64) -> Self {
		// This implementation assumes it happens before the number is big enough that it no longer fits.
		let num = self.mult * old_worry_mod + self.basis;
		Self::new(num, new_worry_mod)
	}
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum OperationType {
	Add,
	Multiply,
}

impl OperationType {
	fn apply(&self, lhs: &WorryAmount, rhs: &WorryAmount, worry_mod: u64) -> WorryAmount {
		match self {
			Self::Add => {
				let mut basis = lhs.basis + rhs.basis;
				let mut mult = lhs.mult + rhs.mult;
				mult += basis / worry_mod;
				basis %= worry_mod;
				mult %= worry_mod;
				WorryAmount { basis, mult }
			}
			Self::Multiply => {
				let mut basis = lhs.basis * rhs.basis;
				let mut mult = (lhs.mult * rhs.basis) + (lhs.basis * rhs.mult);
				mult += basis / worry_mod;
				basis %= worry_mod;
				mult %= worry_mod;
				WorryAmount { basis, mult }
			}
		}
	}
}

#[derive(Clone, Copy)]
enum OperationBy {
	OldValue,
	Number(WorryAmount),
}

#[derive(Clone, Copy)]
struct Operation {
	op: OperationType,
	by: OperationBy,
}

impl Operation {
	fn apply(&self, operand: &WorryAmount, worry_mod: u64) -> WorryAmount {
		let by = match &self.by {
			OperationBy::OldValue => operand,
			OperationBy::Number(num) => num,
		};
		self.op.apply(by, operand, worry_mod)
	}
}

struct Monkey {
	items: Vec<WorryAmount>,
	worry_operation: Operation,
	test_modulus: u64,
	true_destination: usize,
	false_destination: usize,
	throws_made: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut monkeys = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut input_lines = input_string.lines().filter(|s| !s.is_empty());

		let mut monkey_items: Vec<WorryAmount> = Vec::new();
		let mut monkey_operation = Operation {
			op: OperationType::Add,
			by: OperationBy::Number(WorryAmount { basis: 0, mult: 0 }),
		};
		let mut test_modulus: u64 = 1;
		let mut true_destination: usize = 0;
		let mut false_destination: usize = 0;

		input_lines.next(); // Eat the first monkey's header

		let mut monkeys: Vec<Monkey> = Vec::new();

		for line in input_lines {
			if line.starts_with("Monkey ") {
				// Assume the monkeys are in order in the input file
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
				monkey_items = items
					.split(", ")
					.map(|item| {
						let item: u64 = item.parse().unwrap();
						WorryAmount::new(item, DEFAULT_WORRY_MOD)
					})
					.collect();
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
					_ => {
						let amount: u64 = amount.parse()?;
						OperationBy::Number(WorryAmount::new(amount, DEFAULT_WORRY_MOD))
					}
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

	let worry_mod: u64 = monkeys.iter().map(|monkey| monkey.test_modulus).product();
	for monkey in monkeys.iter_mut() {
		for item in monkey.items.iter_mut() {
			*item = item.rebalance(DEFAULT_WORRY_MOD, worry_mod);
		}
	}

	let mut append_to_true: Vec<WorryAmount> = Vec::new();
	let mut append_to_false: Vec<WorryAmount> = Vec::new();

	for _ in 0..10000 {
		for monkey_index in 0..monkeys.len() {
			for item in monkeys[monkey_index].items.iter() {
				let new_worry = monkeys[monkey_index].worry_operation.apply(item, worry_mod);
				if new_worry.is_divisible(monkeys[monkey_index].test_modulus, worry_mod) {
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

	let mut monkey_throws: BinaryHeap<usize> = monkeys.iter().map(|monkey| monkey.throws_made).collect();
	let most_throws = monkey_throws.pop().unwrap();
	let second_most_throws = monkey_throws.pop().unwrap();

	let monkey_business = most_throws * second_most_throws;
	println!("{}", monkey_business);

	Ok(())
}
