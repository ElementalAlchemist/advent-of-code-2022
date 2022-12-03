use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn item_priority(item: char) -> u32 {
	if item.is_ascii_lowercase() {
		(item as u32) - 96
	} else if item.is_ascii_uppercase() {
		(item as u32) - 38
	} else {
		unreachable!()
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let rucksacks: Vec<(HashSet<char>, HashSet<char>)> = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut rucksacks: Vec<(HashSet<char>, HashSet<char>)> = Vec::new();

		for line in input_string.lines().filter(|s| !s.is_empty()) {
			let compartment_border = line.len() / 2;
			let first_compartment: HashSet<char> = line.chars().take(compartment_border).collect();
			let second_compartment: HashSet<char> = line.chars().skip(compartment_border).collect();
			rucksacks.push((first_compartment, second_compartment));
		}

		rucksacks
	};

	let invalid_items: Vec<char> = rucksacks
		.iter()
		.map(|(first, second)| {
			let mut overlap = first.intersection(second);
			let item = overlap.next().unwrap();
			*item
		})
		.collect();

	let answer: u32 = invalid_items.iter().map(|c| item_priority(*c)).sum();
	println!("{}", answer);

	Ok(())
}
