use std::error::Error;
use std::fs;

struct Range {
	start: usize,
	end: usize,
}

impl Range {
	fn overlap(&self, other: &Self) -> bool {
		self.start <= other.end && self.end >= other.start
	}
}

struct ElfPair {
	first: Range,
	second: Range,
}

fn main() -> Result<(), Box<dyn Error>> {
	let elf_pairs: Vec<ElfPair> = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut pair: Vec<ElfPair> = Vec::new();
		for line in input_string.lines().filter(|s| !s.is_empty()) {
			let mut parts = line.split(',');
			let first_range = parts.next().unwrap();
			let second_range = parts.next().unwrap();

			let mut first_parts = first_range.split('-');
			let first_start = first_parts.next().unwrap();
			let first_end = first_parts.next().unwrap();

			let mut second_parts = second_range.split('-');
			let second_start = second_parts.next().unwrap();
			let second_end = second_parts.next().unwrap();

			let first_range = Range {
				start: first_start.parse()?,
				end: first_end.parse()?,
			};
			let second_range = Range {
				start: second_start.parse()?,
				end: second_end.parse()?,
			};

			pair.push(ElfPair {
				first: first_range,
				second: second_range,
			});
		}
		pair
	};

	let output = elf_pairs.iter().filter(|p| p.first.overlap(&p.second)).count();
	println!("{}", output);

	Ok(())
}
