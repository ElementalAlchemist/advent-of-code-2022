use std::error::Error;
use std::fs;
use std::str::FromStr;

struct Instruction {
	count: usize,
	from: usize,
	to: usize,
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut pieces = s.split(' ');
		assert_eq!(pieces.next(), Some("move"));
		let count = pieces.next().unwrap().parse().unwrap();
		assert_eq!(pieces.next(), Some("from"));
		let from: usize = pieces.next().unwrap().parse().unwrap();
		assert_eq!(pieces.next(), Some("to"));
		let to: usize = pieces.next().unwrap().parse().unwrap();

		let from = from - 1;
		let to = to - 1;
		Ok(Self { count, from, to })
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let (mut stacks, instructions) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut stacks: Vec<Vec<char>> = Vec::new();
		let mut instructions: Vec<Instruction> = Vec::new();
		let mut reading_instructions = false;

		for line in input_string.lines() {
			if line.is_empty() {
				reading_instructions = true;
				continue;
			}

			if reading_instructions {
				instructions.push(line.parse().unwrap());
			} else {
				let num_crate_spots = (line.len() + 1) / 4;
				for crate_spot in 0..num_crate_spots {
					let crate_type = line.chars().nth(crate_spot * 4 + 1).unwrap();
					if crate_type == ' ' {
						continue;
					}
					if crate_type.is_numeric() {
						break; // This is the stack number row
					}
					while stacks.len() <= crate_spot {
						stacks.push(Vec::new());
					}
					stacks[crate_spot].insert(0, crate_type);
				}
			}
		}

		(stacks, instructions)
	};

	for instruction in instructions {
		for _ in 0..instruction.count {
			let item = stacks[instruction.from].pop().unwrap();
			while stacks.len() <= instruction.to {
				stacks.push(Vec::new());
			}
			stacks[instruction.to].push(item);
		}
	}

	let mut output = String::new();
	for stack in stacks {
		output.push(*stack.last().unwrap());
	}
	println!("{}", output);

	Ok(())
}
