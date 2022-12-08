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

	let mut visible_trees: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

	for row_index in 0..grid.len() {
		let mut highest_tree = 0;
		for col_index in 0..grid[row_index].len() {
			if grid[row_index][col_index] > highest_tree {
				highest_tree = grid[row_index][col_index];
				visible_trees[row_index][col_index] = true;
			}
		}

		highest_tree = 0;
		for col_index in (0..grid[row_index].len()).rev() {
			if grid[row_index][col_index] > highest_tree {
				highest_tree = grid[row_index][col_index];
				visible_trees[row_index][col_index] = true;
			}
		}
	}

	for col_index in 0..grid[0].len() {
		let mut highest_tree = 0;
		for row_index in 0..grid.len() {
			if grid[row_index][col_index] > highest_tree {
				highest_tree = grid[row_index][col_index];
				visible_trees[row_index][col_index] = true;
			}
		}

		highest_tree = 0;
		for row_index in (0..grid.len()).rev() {
			if grid[row_index][col_index] > highest_tree {
				highest_tree = grid[row_index][col_index];
				visible_trees[row_index][col_index] = true;
			}
		}
	}

	for row in visible_trees.iter_mut() {
		// each column
		row[0] = true;
		let row_len = row.len();
		row[row_len - 1] = true;
	}

	for col in visible_trees[0].iter_mut() {
		*col = true;
	}

	let visible_trees_rows = visible_trees.len();
	for col in visible_trees[visible_trees_rows - 1].iter_mut() {
		*col = true;
	}

	let total: usize = visible_trees
		.iter()
		.map(|row| row.iter().filter(|val| **val).count())
		.sum();
	println!("{}", total);

	Ok(())
}
