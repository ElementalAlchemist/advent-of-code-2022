use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let grid: Vec<Vec<u8>> = {
		let input_string = fs::read_to_string("input.txt")?;
		let data: Vec<Vec<u8>> = input_string
			.lines()
			.map(|line| line.chars().map(|c| (c as u8) - 48).collect())
			.collect();
		data
	};

	let mut highest_scenic_score = 0;
	for (row_index, row) in grid.iter().enumerate() {
		for (col_index, height) in row.iter().enumerate() {
			let mut up_score = 0;
			for other_row in grid.iter().take(row_index).rev() {
				up_score += 1;
				let other_height = other_row[col_index];
				if other_height >= *height {
					break;
				}
			}

			let mut left_score = 0;
			for other_height in row.iter().take(col_index).rev() {
				left_score += 1;
				if *other_height >= *height {
					break;
				}
			}

			let mut down_score = 0;
			for other_row in grid.iter().skip(row_index + 1) {
				down_score += 1;
				let other_height = other_row[col_index];
				if other_height >= *height {
					break;
				}
			}

			let mut right_score = 0;
			for other_height in row.iter().skip(col_index + 1) {
				right_score += 1;
				if *other_height >= *height {
					break;
				}
			}

			let scenic_score = up_score * left_score * down_score * right_score;
			if scenic_score > highest_scenic_score {
				println!("{} {}: {} {} {} {}", row_index, col_index, up_score, left_score, down_score, right_score);
				highest_scenic_score = scenic_score;
			}
		}
	}

	println!("{}", highest_scenic_score);

	Ok(())
}
