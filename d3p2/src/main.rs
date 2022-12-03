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

	let mut total_priority = 0;
	for group in 0..(rucksacks.len() / 3) {
		let start_index = group * 3;
		let first_group: HashSet<char> = rucksacks[start_index]
			.0
			.union(&rucksacks[start_index].1)
			.copied()
			.collect();
		let second_group: HashSet<char> = rucksacks[start_index + 1]
			.0
			.union(&rucksacks[start_index + 1].1)
			.copied()
			.collect();
		let third_group: HashSet<char> = rucksacks[start_index + 2]
			.0
			.union(&rucksacks[start_index + 2].1)
			.copied()
			.collect();

		let first_two: HashSet<char> = first_group.intersection(&second_group).copied().collect();
		let mut remaining = first_two.intersection(&third_group);
		let badge = *remaining.next().unwrap();
		total_priority += item_priority(badge);
	}

	println!("{}", total_priority);

	Ok(())
}
