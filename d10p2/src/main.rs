use std::error::Error;
use std::fs;
use std::str::FromStr;

enum Instruction {
	Add(i32),
	NoOp,
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split(' ');
		let instruction = parts.next().unwrap();
		match instruction {
			"addx" => {
				let addend: i32 = parts.next().unwrap().parse().unwrap();
				Ok(Self::Add(addend))
			}
			"noop" => Ok(Self::NoOp),
			_ => unreachable!(),
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let instructions: Vec<Instruction> = {
		let input_string = fs::read_to_string("input.txt")?;
		input_string.lines().map(|s| s.parse().unwrap()).collect()
	};

	let mut x: i32 = 1;
	let mut mid_cycle_addend: Option<i32> = None;
	let mut cycle_number = 0;
	let mut instruction_iter = instructions.iter();

	loop {
		cycle_number += 1;

		if let Some(addend) = mid_cycle_addend.take() {
			x += addend;
		} else {
			let Some(next_instruction) = instruction_iter.next() else {
				break;
			};
			match next_instruction {
				Instruction::Add(num) => mid_cycle_addend = Some(*num),
				Instruction::NoOp => (),
			}
		}

		if x.abs_diff(cycle_number % 40) <= 1 {
			print!("#");
		} else {
			print!(".");
		}
		if cycle_number % 40 == 0 {
			println!();
		}
	}

	Ok(())
}
