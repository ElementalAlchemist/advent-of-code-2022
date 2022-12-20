use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let orig_file_data: Vec<i32> = {
		let input_string = fs::read_to_string("input.txt")?;
		let mut data = Vec::new();
		for line in input_string.lines().filter(|s| !s.is_empty()) {
			data.push(line.parse()?);
		}
		data
	};

	let mut new_indices: HashMap<usize, usize> = orig_file_data
		.iter()
		.enumerate()
		.map(|(index, _)| (index, index))
		.collect();
	let file_len = orig_file_data.len();
	let data_len: i32 = file_len.try_into().unwrap();

	for (index, val) in orig_file_data.iter().enumerate() {
		let mut adjustment = *val;
		adjustment = adjustment.rem_euclid(data_len - 1);
		let adjustment: usize = adjustment.try_into().unwrap();
		let index_value = new_indices.get(&index).unwrap();
		let old_value = *index_value;
		let mut new_value = *index_value + adjustment;
		if new_value >= file_len {
			new_value += 1;
		}
		new_value = new_value.rem_euclid(file_len);
		if old_value > new_value {
			for new_index in new_indices.values_mut() {
				if (new_value..old_value).contains(new_index) {
					*new_index = (*new_index + 1).rem_euclid(file_len);
				}
			}
		} else {
			for new_index in new_indices.values_mut() {
				if (old_value..=new_value).contains(new_index) {
					*new_index = (*new_index + file_len - 1).rem_euclid(file_len);
				}
			}
		}
		let index_value = new_indices.get_mut(&index).unwrap();
		*index_value = new_value;
	}

	let mut file_data = vec![0; orig_file_data.len()];
	for (orig_index, new_index) in new_indices.iter() {
		file_data[*new_index] = orig_file_data[*orig_index];
	}

	assert_eq!(file_data.iter().filter(|val| **val == 0).count(), 1);
	let zero_pos = file_data.iter().enumerate().find(|(_, val)| **val == 0).unwrap().0;
	let grove1 = file_data[(1000 + zero_pos) % file_len];
	let grove2 = file_data[(2000 + zero_pos) % file_len];
	let grove3 = file_data[(3000 + zero_pos) % file_len];
	let grove_total = grove1 + grove2 + grove3;
	println!("{} + {} + {} = {}", grove1, grove2, grove3, grove_total);

	Ok(())
}
