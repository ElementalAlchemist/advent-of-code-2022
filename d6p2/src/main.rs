use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let data: Vec<char> = fs::read_to_string("input.txt")?.chars().collect();

	'value_loop: for values in data.windows(14).enumerate() {
		let mut charset: HashSet<char> = HashSet::new();
		charset.insert(values.1[0]);
		for index in 1..14 {
			if !charset.insert(values.1[index]) {
				continue 'value_loop;
			}
		}

		println!("{}", values.0 + 14);
		break;
	}

	Ok(())
}
