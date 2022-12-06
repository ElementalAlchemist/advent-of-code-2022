use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let data: Vec<char> = fs::read_to_string("input.txt")?.chars().collect();

	for values in data.windows(4).enumerate() {
		let mut charset: HashSet<char> = HashSet::new();
		charset.insert(values.1[0]);
		if !charset.insert(values.1[1]) {
			continue;
		}
		if !charset.insert(values.1[2]) {
			continue;
		}
		if !charset.insert(values.1[3]) {
			continue;
		}

		println!("{}", values.0 + 4);
		break;
	}

	Ok(())
}
